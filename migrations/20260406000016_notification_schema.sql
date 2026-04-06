-- BANKO Notification BC (STORY-NOT-03 & STORY-NOT-04)
-- Creates notification schema with tables for templates, preferences, and the async queue

CREATE SCHEMA IF NOT EXISTS notifications;

-- Notification Templates
CREATE TABLE notifications.notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    channel VARCHAR(20) NOT NULL CHECK (channel IN ('Email', 'Sms', 'Push')),
    locale VARCHAR(5) NOT NULL DEFAULT 'fr',
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables_schema JSONB DEFAULT '[]',
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(event_type, channel, locale)
);

CREATE INDEX idx_notification_templates_event_type ON notifications.notification_templates(event_type);
CREATE INDEX idx_notification_templates_channel ON notifications.notification_templates(channel);
CREATE INDEX idx_notification_templates_active ON notifications.notification_templates(active);

-- Notification Preferences
CREATE TABLE notifications.notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    channel VARCHAR(20) NOT NULL CHECK (channel IN ('Email', 'Sms', 'Push')),
    notification_type VARCHAR(20) NOT NULL CHECK (notification_type IN ('Transactional', 'Security', 'Regulatory', 'Marketing')),
    opted_in BOOLEAN NOT NULL DEFAULT true,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(customer_id, channel, notification_type)
);

CREATE INDEX idx_notification_preferences_customer_id ON notifications.notification_preferences(customer_id);
CREATE INDEX idx_notification_preferences_channel ON notifications.notification_preferences(channel);

-- Notifications Queue
CREATE TABLE notifications.notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    channel VARCHAR(20) NOT NULL CHECK (channel IN ('Email', 'Sms', 'Push')),
    notification_type VARCHAR(20) NOT NULL CHECK (notification_type IN ('Transactional', 'Security', 'Regulatory', 'Marketing')),
    template_id VARCHAR(100) NOT NULL,
    variables JSONB DEFAULT '{}',
    recipient VARCHAR(255) NOT NULL,
    subject TEXT,
    body TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Queued', 'Sending', 'Sent', 'Delivered', 'Failed', 'Retrying')),
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ
);

CREATE INDEX idx_notifications_customer_id ON notifications.notifications(customer_id);
CREATE INDEX idx_notifications_status ON notifications.notifications(status);
CREATE INDEX idx_notifications_created_at ON notifications.notifications(created_at);
CREATE INDEX idx_notifications_channel ON notifications.notifications(channel);

-- Seed default templates for key banking events
INSERT INTO notifications.notification_templates (event_type, channel, locale, subject_template, body_template) VALUES
('account_opened', 'Email', 'fr', 'Votre compte {{account_type}} est ouvert', 'Bonjour {{client_name}}, votre compte {{account_type}} n°{{account_number}} a été ouvert avec succès.'),
('account_opened', 'Email', 'ar', 'تم فتح حسابك {{account_type}}', 'مرحبا {{client_name}}، تم فتح حسابك {{account_type}} رقم {{account_number}} بنجاح.'),
('account_opened', 'Email', 'en', 'Your {{account_type}} account is open', 'Hello {{client_name}}, your {{account_type}} account #{{account_number}} has been successfully opened.'),
('payment_received', 'Email', 'fr', 'Paiement reçu de {{amount}} {{currency}}', 'Bonjour {{client_name}}, un paiement de {{amount}} {{currency}} a été reçu sur votre compte.'),
('payment_received', 'Sms', 'fr', NULL, 'BANKO: Paiement reçu {{amount}} {{currency}} sur votre compte. Ref: {{reference}}'),
('kyc_approved', 'Email', 'fr', 'Votre KYC a été approuvé', 'Bonjour {{client_name}}, votre vérification d''identité (KYC) a été approuvée.'),
('aml_alert', 'Email', 'fr', 'Alerte AML - Action requise', 'Une alerte anti-blanchiment a été détectée. Référence: {{alert_id}}. Veuillez examiner.'),
('loan_approved', 'Email', 'fr', 'Votre prêt a été approuvé', 'Bonjour {{client_name}}, votre demande de prêt de {{amount}} {{currency}} a été approuvée.'),
('security_alert', 'Email', 'fr', 'Alerte de sécurité sur votre compte', 'Bonjour {{client_name}}, une activité inhabituelle a été détectée sur votre compte. {{details}}'),
('security_alert', 'Sms', 'fr', NULL, 'BANKO SECURITE: Activité inhabituelle détectée. Si ce n''est pas vous, appelez le 71 000 000.')
ON CONFLICT DO NOTHING;
