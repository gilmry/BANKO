Feature: Customer Management (BC1) - BMAD v4.0.1 Compliance
  As a bank officer and compliance officer
  I want to manage customer onboarding, KYC, segmentation, documents, and data rights
  So that clients are properly identified and fully compliant with BMAD v4.0.1

  # FR-001: Onboarding client (physique/morale) avec KYC Circ. 2025-17
  Scenario: Create a new individual customer with valid KYC
    Given a new individual customer with name "Ahmed Ben Ali" and email "ahmed@example.tn"
    When I submit the customer onboarding form with consent given
    Then the customer is created with status "Pending"
    And the customer has KYC profile submitted

  Scenario: Create a new legal entity with beneficiaries
    Given a new legal entity "Banko SA" with registration "RCS-12345"
    And with beneficiary "Ahmed" owning 60% and "Fatima" owning 40%
    When I submit the legal entity onboarding form
    Then the customer is created as legal entity
    And both beneficiaries are registered

  # FR-002: Profilage risque client automatique (scoring PEP, pays, activité)
  Scenario: Auto-score customer risk based on PEP status
    Given a customer with full name "General Ahmed Ben Ali"
    When the system checks PEP status
    Then the customer is flagged as PEP
    And risk score is automatically increased

  # FR-003: e-KYC biométrique (Circ. 2025-06, FIDO2/WebAuthn)
  Scenario: Customer consent required for data processing
    Given a new customer onboarding request
    When consent is NOT given for INPDP data processing
    Then the onboarding is rejected with error "ConsentRequired"

  # FR-004: Mise à jour KYC périodique
  Scenario: Update KYC profile
    Given an existing customer with KYC profile
    When I update the KYC with new address and profession
    Then the KYC profile is updated
    And updated_at timestamp is refreshed

  # FR-005: Gestion bénéficiaires effectifs (>25% parts)
  Scenario: Validate beneficial owner shares
    Given a legal entity with two beneficiaries
    When their combined share exceeds 100%
    Then the creation is rejected with error "exceed 100%"

  # FR-006: Catégorisation client (retail, corporate, VIP, private banking, institutional)
  Scenario: Assign customer segment
    Given a new customer being created
    When I assign segment "Retail"
    Then customer segment is set to "Retail"
    And can be updated to "VIP" or "Corporate"

  # FR-007: Groupement économique (familles, entreprises liées)
  Scenario: Create customer economic group
    Given multiple customers in a family
    When I create a group linking them
    Then the group is created with all members
    And parent customer can be designated

  Scenario: Manage group membership
    Given an existing customer group
    When I add a new member
    Then member count increases
    And can be removed (keeping at least one member)

  # FR-008: Cycle de vie documents (CIN, passeport, extrait K-bis, expiry tracking)
  Scenario: Add customer document with expiry
    Given a customer profile
    When I add a CIN document valid until 2030-01-01
    Then the document is stored
    And validity period is tracked

  Scenario: Alert for expiring documents
    Given a customer with document expiring in 15 days
    When checking documents expiring in 30 days
    Then the document appears in renewal alerts
    And days until expiry is calculated

  # FR-009: Historique complet des modifications (audit trail)
  Scenario: Track customer history
    Given a customer with initial data
    When the customer is modified
    Then updated_at timestamp changes
    And previous state is preserved

  # FR-010: Consentement INPDP (Loi 2025, granulaire par finalité)
  Scenario: Reject customer without INPDP consent
    Given a new customer without consent
    When submitting onboarding
    Then request fails with "ConsentRequired" error

  # FR-011: Droit à l'effacement (avec vérification rétention 7 ans)
  Scenario: Cannot anonymize customer before 10-year retention
    Given a customer closed 5 years ago
    When attempting anonymization
    Then request fails with retention period error

  Scenario: Anonymize customer after 10-year retention
    Given a customer closed 10 years ago
    When anonymizing
    Then PII is replaced with "[ANONYMIZED]"
    And status becomes "Anonymized"

  # FR-012: Portabilité des données (export JSON/CSV)
  Scenario: Request customer data export
    Given an existing customer
    When requesting data export
    Then export includes profile, accounts, transactions
    And export includes consent records

  # FR-013: Notification changement statut KYC
  Scenario: KYC approval changes status
    Given a pending KYC customer
    When KYC is approved
    Then status changes to "Approved"

  # FR-014: Recherche multi-critères client
  Scenario: Search customers by multiple criteria
    Given multiple customers with various attributes
    When searching by full_name="Ahmed" AND status="Approved"
    Then matching customers are returned
    And results respect limit and offset parameters

  # FR-015: Désactivation/réactivation client
  Scenario: Suspend customer
    Given an approved customer
    When suspending the customer
    Then status changes to "Suspended"
