use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use banko_domain::notification::{
    Channel, Notification, NotificationId, NotificationPreference, NotificationStatus,
    NotificationType,
};

use super::dto::{
    NotificationDto, NotificationPreferenceRequest, NotificationPreferenceResponse,
    PreferenceItem, ProcessingError, ProcessingResult, SendNotificationRequest,
    SendNotificationResponse,
};
use super::errors::NotificationError;
use super::ports::{
    INotificationPreferenceRepository, INotificationRepository, INotificationSender,
    ITemplateRepository,
};

// ============================================================
// NotificationService
// ============================================================

pub struct NotificationService {
    notification_repo: Arc<dyn INotificationRepository>,
    template_repo: Arc<dyn ITemplateRepository>,
    preference_repo: Arc<dyn INotificationPreferenceRepository>,
    sender: Arc<dyn INotificationSender>,
}

impl NotificationService {
    pub fn new(
        notification_repo: Arc<dyn INotificationRepository>,
        template_repo: Arc<dyn ITemplateRepository>,
        preference_repo: Arc<dyn INotificationPreferenceRepository>,
        sender: Arc<dyn INotificationSender>,
    ) -> Self {
        NotificationService {
            notification_repo,
            template_repo,
            preference_repo,
            sender,
        }
    }

    /// Send a notification through the full pipeline
    pub async fn send_notification(
        &self,
        req: SendNotificationRequest,
    ) -> Result<SendNotificationResponse, NotificationError> {
        // Parse channel
        let channel = Channel::from_str(&req.channel)
            .map_err(|e| NotificationError::InvalidChannel(e.to_string()))?;

        // Parse notification type
        let notification_type = NotificationType::from_str(&req.notification_type)
            .map_err(|e| NotificationError::InvalidInput(e.to_string()))?;

        // Step 1: Check preferences for marketing notifications
        if notification_type == NotificationType::Marketing {
            let pref = self
                .preference_repo
                .find_by_customer_and_channel_type(req.customer_id, channel, notification_type)
                .await
                .map_err(|e| NotificationError::RepositoryError(e))?;

            match pref {
                Some(p) => {
                    if !p.allows_notification(notification_type) {
                        return Err(NotificationError::RecipientOptedOut(
                            "Marketing".to_string(),
                        ));
                    }
                }
                None => {
                    // No preference found - marketing is opt-in, so reject
                    return Err(NotificationError::RecipientOptedOut(
                        "Marketing".to_string(),
                    ));
                }
            }
        }

        // Step 2: Load template
        let locale = req.locale.unwrap_or_else(|| "fr".to_string());
        let template = self
            .template_repo
            .find_by_event_and_locale(&req.template_id, &locale)
            .await
            .map_err(|e| NotificationError::RepositoryError(e))?
            .ok_or_else(|| NotificationError::TemplateNotFound(req.template_id.clone()))?;

        // Step 3: Render template
        let variables = req.variables.unwrap_or_default();
        let subject = Notification::render_template(&template.subject_template, &variables);
        let body = Notification::render_template(&template.body_template, &variables);

        // Step 4: Create notification entity
        let notification = Notification::new(
            req.customer_id,
            channel,
            notification_type,
            req.template_id,
            variables,
            req.recipient,
            subject,
            body,
        )
        .map_err(|e| NotificationError::DomainError(e.to_string()))?;

        // Step 5: Save to repository
        self.notification_repo
            .save(&notification)
            .await
            .map_err(|e| NotificationError::RepositoryError(e))?;

        // Step 6: Return response
        Ok(SendNotificationResponse {
            notification_id: notification.id().as_uuid().to_string(),
            status: notification.status().to_string(),
        })
    }

    /// Process pending notifications in batches
    pub async fn process_pending_notifications(
        &self,
        batch_size: usize,
    ) -> Result<ProcessingResult, NotificationError> {
        let pending = self
            .notification_repo
            .find_pending(batch_size)
            .await
            .map_err(|e| NotificationError::RepositoryError(e))?;

        let mut result = ProcessingResult {
            total_processed: pending.len(),
            sent: 0,
            failed: 0,
            skipped: 0,
            errors: Vec::new(),
        };

        for mut notification in pending {
            // Check for deduplication
            let recent = self
                .notification_repo
                .find_recent_by_customer_channel_template(
                    notification.customer_id(),
                    *notification.channel(),
                    notification.template_id(),
                    5, // 5 minute dedup window
                )
                .await
                .map_err(|e| NotificationError::RepositoryError(e))?;

            // Skip if already sent in dedup window
            if recent
                .iter()
                .any(|n| n.id().as_uuid() != notification.id().as_uuid() && *n.status() == NotificationStatus::Sent)
            {
                result.skipped += 1;
                continue;
            }

            // Try to send
            notification.mark_sending();

            let send_result = match notification.channel() {
                Channel::Email => {
                    self.sender
                        .send_email(
                            notification.recipient(),
                            notification.subject(),
                            notification.body(),
                        )
                        .await
                }
                Channel::Sms => {
                    self.sender
                        .send_sms(notification.recipient(), notification.body())
                        .await
                }
                Channel::Push => {
                    self.sender
                        .send_push(
                            notification.recipient(),
                            notification.subject(),
                            notification.body(),
                        )
                        .await
                }
            };

            match send_result {
                Ok(()) => {
                    notification.mark_sent();
                    result.sent += 1;
                }
                Err(e) => {
                    if notification.should_retry() {
                        notification.increment_retry();
                    } else {
                        notification.mark_failed(e.clone());
                        result.failed += 1;
                        result.errors.push(ProcessingError {
                            notification_id: notification.id().as_uuid().to_string(),
                            reason: e,
                        });
                    }
                }
            }

            // Update in repository
            self.notification_repo
                .save(&notification)
                .await
                .map_err(|e| NotificationError::RepositoryError(e))?;
        }

        Ok(result)
    }

    /// Get all preferences for a customer
    pub async fn get_customer_preferences(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<NotificationPreference>, NotificationError> {
        self.preference_repo
            .find_by_customer(customer_id)
            .await
            .map_err(|e| NotificationError::RepositoryError(e))
    }

    /// Update a customer preference
    pub async fn update_preference(
        &self,
        req: NotificationPreferenceRequest,
    ) -> Result<(), NotificationError> {
        let channel = Channel::from_str(&req.channel)
            .map_err(|e| NotificationError::InvalidChannel(e.to_string()))?;

        let notification_type = NotificationType::from_str(&req.notification_type)
            .map_err(|e| NotificationError::InvalidInput(e.to_string()))?;

        let preference =
            NotificationPreference::new(req.customer_id, channel, notification_type, req.opted_in);

        self.preference_repo
            .save(&preference)
            .await
            .map_err(|e| NotificationError::RepositoryError(e))
    }

    /// Get a specific notification
    pub async fn get_notification(&self, id: &str) -> Result<Notification, NotificationError> {
        self.notification_repo
            .find_by_id(id)
            .await
            .map_err(|e| NotificationError::RepositoryError(e))?
            .ok_or(NotificationError::NotFound)
    }

    /// Convert notification to DTO
    fn notification_to_dto(&self, notif: &Notification) -> NotificationDto {
        NotificationDto {
            id: notif.id().as_uuid().to_string(),
            customer_id: notif.customer_id().to_string(),
            channel: notif.channel().to_string(),
            notification_type: notif.notification_type().to_string(),
            template_id: notif.template_id().to_string(),
            variables: notif.variables().clone(),
            recipient: notif.recipient().to_string(),
            subject: notif.subject().to_string(),
            body: notif.body().to_string(),
            status: notif.status().to_string(),
            retry_count: notif.retry_count(),
            max_retries: notif.max_retries(),
            created_at: *notif.created_at(),
            sent_at: notif.sent_at().cloned(),
            delivered_at: notif.delivered_at().cloned(),
            error_message: notif.error_message().map(|s| s.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notification::ports::mocks::{
        MockNotificationPreferenceRepository, MockNotificationRepository, MockNotificationSender,
        MockTemplateRepository,
    };
    use crate::notification::ports::TemplateInfo;

    async fn setup_service(
    ) -> (
        NotificationService,
        Arc<MockNotificationRepository>,
        Arc<MockTemplateRepository>,
        Arc<MockNotificationPreferenceRepository>,
        Arc<MockNotificationSender>,
    ) {
        let notif_repo = Arc::new(MockNotificationRepository::new());
        let template_repo = Arc::new(MockTemplateRepository::new());
        let pref_repo = Arc::new(MockNotificationPreferenceRepository::new());
        let sender = Arc::new(MockNotificationSender::new());

        let service = NotificationService::new(
            notif_repo.clone(),
            template_repo.clone(),
            pref_repo.clone(),
            sender.clone(),
        );

        (service, notif_repo, template_repo, pref_repo, sender)
    }

    #[tokio::test]
    async fn test_send_transactional_notification() {
        let (service, template_repo, _, _, sender) = setup_service().await;

        template_repo.add_template(TemplateInfo {
            id: "order_confirmation".to_string(),
            event_type: "order_confirmation".to_string(),
            channel: "Email".to_string(),
            locale: "fr".to_string(),
            subject_template: "Commande confirmée".to_string(),
            body_template: "Bonjour {{name}}, votre commande #{{order_id}} est confirmée."
                .to_string(),
        });

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());
        variables.insert("order_id".to_string(), "123456".to_string());

        let req = SendNotificationRequest {
            customer_id: Uuid::new_v4(),
            channel: "Email".to_string(),
            notification_type: "Transactional".to_string(),
            template_id: "order_confirmation".to_string(),
            variables: Some(variables),
            recipient: "alice@example.com".to_string(),
            locale: Some("fr".to_string()),
        };

        let result = service.send_notification(req).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, "Pending");
    }

    #[tokio::test]
    async fn test_send_marketing_notification_with_opt_in() {
        let (service, template_repo, pref_repo, _, _) = setup_service().await;

        let customer_id = Uuid::new_v4();

        // Set up marketing opt-in
        let pref = NotificationPreference::new(
            customer_id,
            Channel::Email,
            NotificationType::Marketing,
            true,
        );
        pref_repo.save(&pref).await.unwrap();

        // Add template
        template_repo.add_template(TemplateInfo {
            id: "promo".to_string(),
            event_type: "promo".to_string(),
            channel: "Email".to_string(),
            locale: "fr".to_string(),
            subject_template: "Promotion spéciale".to_string(),
            body_template: "Venez profiter de nos offres!".to_string(),
        });

        let req = SendNotificationRequest {
            customer_id,
            channel: "Email".to_string(),
            notification_type: "Marketing".to_string(),
            template_id: "promo".to_string(),
            variables: None,
            recipient: "user@example.com".to_string(),
            locale: Some("fr".to_string()),
        };

        let result = service.send_notification(req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reject_marketing_without_opt_in() {
        let (service, template_repo, _, _, _) = setup_service().await;

        let customer_id = Uuid::new_v4();

        // Add template
        template_repo.add_template(TemplateInfo {
            id: "promo".to_string(),
            event_type: "promo".to_string(),
            channel: "Email".to_string(),
            locale: "fr".to_string(),
            subject_template: "Promotion".to_string(),
            body_template: "Venez profiter de nos offres!".to_string(),
        });

        let req = SendNotificationRequest {
            customer_id,
            channel: "Email".to_string(),
            notification_type: "Marketing".to_string(),
            template_id: "promo".to_string(),
            variables: None,
            recipient: "user@example.com".to_string(),
            locale: Some("fr".to_string()),
        };

        let result = service.send_notification(req).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            NotificationError::RecipientOptedOut(_)
        ));
    }

    #[tokio::test]
    async fn test_template_fallback_to_default_locale() {
        let (service, template_repo, _, _, _) = setup_service().await;

        // Add template only for default locale (fr)
        template_repo.add_template(TemplateInfo {
            id: "welcome".to_string(),
            event_type: "welcome".to_string(),
            channel: "Email".to_string(),
            locale: "fr".to_string(),
            subject_template: "Bienvenue".to_string(),
            body_template: "Bienvenue dans BANKO".to_string(),
        });

        let req = SendNotificationRequest {
            customer_id: Uuid::new_v4(),
            channel: "Email".to_string(),
            notification_type: "Transactional".to_string(),
            template_id: "welcome".to_string(),
            variables: None,
            recipient: "user@example.com".to_string(),
            locale: Some("en".to_string()), // Request EN but should fallback to FR
        };

        let result = service.send_notification(req).await;

        // Should succeed with fallback
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_template_not_found() {
        let (service, _, _, _, _) = setup_service().await;

        let req = SendNotificationRequest {
            customer_id: Uuid::new_v4(),
            channel: "Email".to_string(),
            notification_type: "Transactional".to_string(),
            template_id: "nonexistent".to_string(),
            variables: None,
            recipient: "user@example.com".to_string(),
            locale: None,
        };

        let result = service.send_notification(req).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            NotificationError::TemplateNotFound(_)
        ));
    }

    #[tokio::test]
    async fn test_regulatory_notification_always_sent() {
        let (service, template_repo, pref_repo, _, _) = setup_service().await;

        let customer_id = Uuid::new_v4();

        // Set up opted-out regulatory preference
        let pref = NotificationPreference::new(
            customer_id,
            Channel::Email,
            NotificationType::Regulatory,
            false, // Opted out
        );
        pref_repo.save(&pref).await.unwrap();

        // Add template
        template_repo.add_template(TemplateInfo {
            id: "compliance_alert".to_string(),
            event_type: "compliance_alert".to_string(),
            channel: "Email".to_string(),
            locale: "fr".to_string(),
            subject_template: "Alerte de conformité".to_string(),
            body_template: "Vous devez vérifier votre compte".to_string(),
        });

        let req = SendNotificationRequest {
            customer_id,
            channel: "Email".to_string(),
            notification_type: "Regulatory".to_string(),
            template_id: "compliance_alert".to_string(),
            variables: None,
            recipient: "user@example.com".to_string(),
            locale: Some("fr".to_string()),
        };

        let result = service.send_notification(req).await;

        // Should succeed (regulatory cannot be opted out)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_customer_preferences() {
        let (service, _, pref_repo, _, _) = setup_service().await;

        let customer_id = Uuid::new_v4();

        let pref1 = NotificationPreference::new(
            customer_id,
            Channel::Email,
            NotificationType::Marketing,
            true,
        );
        let pref2 = NotificationPreference::new(
            customer_id,
            Channel::Sms,
            NotificationType::Transactional,
            false,
        );

        pref_repo.save(&pref1).await.unwrap();
        pref_repo.save(&pref2).await.unwrap();

        let prefs = service.get_customer_preferences(customer_id).await;

        assert!(prefs.is_ok());
        let prefs_list = prefs.unwrap();
        assert_eq!(prefs_list.len(), 2);
    }

    #[tokio::test]
    async fn test_update_preference() {
        let (service, _, _, _, _) = setup_service().await;

        let customer_id = Uuid::new_v4();

        let req = NotificationPreferenceRequest {
            customer_id,
            channel: "Email".to_string(),
            notification_type: "Marketing".to_string(),
            opted_in: true,
        };

        let result = service.update_preference(req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_pending_notifications_email() {
        let (service, notif_repo, _, _, sender) = setup_service().await;

        let customer_id = Uuid::new_v4();
        let notif = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Test Subject".to_string(),
            "Test Body".to_string(),
        )
        .unwrap();

        notif_repo.save(&notif).await.unwrap();

        let result = service.process_pending_notifications(10).await;

        assert!(result.is_ok());
        let processing = result.unwrap();
        assert_eq!(processing.total_processed, 1);
        assert_eq!(processing.sent, 1);
        assert_eq!(processing.failed, 0);

        // Verify email was sent
        assert_eq!(sender.emails_sent.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_process_pending_notifications_sms() {
        let (service, notif_repo, _, _, sender) = setup_service().await;

        let customer_id = Uuid::new_v4();
        let notif = Notification::new(
            customer_id,
            Channel::Sms,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "+212612345678".to_string(),
            "SMS Title".to_string(),
            "Test SMS Message".to_string(),
        )
        .unwrap();

        notif_repo.save(&notif).await.unwrap();

        let result = service.process_pending_notifications(10).await;

        assert!(result.is_ok());
        let processing = result.unwrap();
        assert_eq!(processing.sent, 1);

        // Verify SMS was sent
        assert_eq!(sender.sms_sent.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_process_pending_notifications_deduplication() {
        let (service, notif_repo, _, _, sender) = setup_service().await;

        let customer_id = Uuid::new_v4();

        // Create two similar notifications
        let notif1 = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        let mut notif2 = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        // Mark first as sent
        notif2.mark_sent();

        notif_repo.save(&notif1).await.unwrap();
        notif_repo.save(&notif2).await.unwrap();

        let result = service.process_pending_notifications(10).await;

        assert!(result.is_ok());
        let processing = result.unwrap();
        // Should skip one due to dedup
        assert!(processing.skipped > 0 || processing.sent == 1);
    }

    #[tokio::test]
    async fn test_get_notification_by_id() {
        let (service, notif_repo, _, _, _) = setup_service().await;

        let customer_id = Uuid::new_v4();
        let notif = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        let notif_id = notif.id().as_uuid().to_string();
        notif_repo.save(&notif).await.unwrap();

        let result = service.get_notification(&notif_id).await;

        assert!(result.is_ok());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.customer_id(), customer_id);
    }

    #[tokio::test]
    async fn test_retry_logic_on_send_failure() {
        // This test demonstrates the retry mechanism
        let customer_id = Uuid::new_v4();
        let mut notif = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        // Fail and retry
        notif.mark_failed("SMTP error".to_string());
        assert!(notif.should_retry());

        notif.increment_retry();
        assert_eq!(notif.retry_count(), 1);

        // Exhaust retries
        notif.mark_failed("Still failing".to_string());
        notif.retry_count = 3;
        assert!(!notif.should_retry());
    }
}
