Feature: ReferenceData Bounded Context
  Centralized reference data management for the banking system

  Background:
    Given the reference data database is initialized
    And the system is ready to accept requests

  # --- Country Code Feature ---
  Feature: Country Code Management

    Scenario: Create a new country code
      When I create a country code with:
        | iso_alpha2 | TN                    |
        | iso_alpha3 | TUN                   |
        | iso_numeric| 788                   |
        | name_en    | Tunisia               |
        | name_fr    | Tunisie               |
        | name_ar    | تونس                  |
        | is_sanctioned | false               |
      Then the country code should be saved successfully
      And I should be able to retrieve it by ISO Alpha-2 "TN"
      And I should be able to retrieve it by ISO Alpha-3 "TUN"

    Scenario: Validate country code format
      When I attempt to create a country code with invalid ISO Alpha-2 "TUN"
      Then the system should reject it with validation error
      And the error message should indicate "ISO Alpha-2 code must be exactly 2 characters"

    Scenario: List active countries
      Given I have created 3 active country codes
      And I have created 1 inactive country code
      When I list active countries
      Then I should receive 3 country codes
      And all returned countries should have is_active = true

    Scenario: Flag a country as sanctioned
      Given I have created a country code for "FR"
      When I update the country code to mark it as sanctioned
      Then the country should be flagged as sanctioned
      And the update timestamp should reflect the change

  # --- Currency Reference Feature ---
  Feature: Currency Reference Management

    Scenario: Create a new currency
      When I create a currency reference with:
        | code | USD                          |
        | name_en | US Dollar                  |
        | name_fr | Dollar américain           |
        | decimal_places | 2                     |
        | is_active | true                      |
      Then the currency reference should be saved successfully
      And I should be able to retrieve it by code "USD"

    Scenario: Validate decimal places constraint
      When I attempt to create a currency with decimal_places = 10
      Then the system should reject it with validation error
      And the error message should indicate "Decimal places must be between 0 and 8"

    Scenario: List active currencies
      Given I have created 5 active currencies
      And I have created 2 inactive currencies
      When I list active currencies
      Then I should receive 5 currencies
      And all returned currencies should have is_active = true

    Scenario: Crypto currency with 8 decimal places
      When I create a currency with code "BTC" and decimal_places = 8
      Then the currency should be saved successfully
      And decimal_places should be exactly 8

  # --- Bank Code Feature ---
  Feature: Bank Code Management

    Scenario: Create a new bank code
      Given I have a country code "FR"
      When I create a bank code with:
        | bic | BNAFFRPP                     |
        | bank_name | BNP Paribas             |
        | country_iso_alpha2 | FR             |
        | is_active | true                     |
      Then the bank code should be saved successfully
      And I should be able to retrieve it by BIC "BNAFFRPP"

    Scenario: BIC code normalization
      When I create a bank code with BIC "bna-ffr-pp"
      Then the BIC should be normalized to "BNAFFRPP" (uppercase, no hyphens)

    Scenario: List active bank codes
      Given I have created 10 active bank codes
      When I list active bank codes
      Then I should receive 10 bank codes
      And all returned banks should have is_active = true

  # --- Branch Code Feature ---
  Feature: Branch Code Management

    Scenario: Create a new branch code
      Given I have a bank code "BNAFFRPP"
      When I create a branch code with:
        | branch_code | 00001                                 |
        | branch_name | Main Branch                           |
        | bank_bic | BNAFFRPP                             |
        | city | Paris                                  |
        | address | 123 Rue de Rivoli, 75001 Paris        |
        | is_active | true                               |
      Then the branch code should be saved successfully
      And I should be able to retrieve it by branch_code "00001"

    Scenario: Find branches by bank BIC
      Given I have created 5 branch codes for bank "BNAFFRPP"
      When I query for branches of bank "BNAFFRPP"
      Then I should receive 5 branch codes
      And all returned branches should have bank_bic = "BNAFFRPP"

  # --- Holiday Calendar Feature ---
  Feature: Holiday Calendar Management

    Scenario: Create a national holiday
      When I create a holiday with:
        | holiday_date | 2026-03-20                           |
        | holiday_name_en | Independence Day                  |
        | holiday_name_fr | Fête de l'Indépendance           |
        | holiday_name_ar | عيد الاستقلال                      |
        | holiday_type | National                            |
        | is_banking_holiday | true                           |
      Then the holiday should be saved successfully

    Scenario: Create a religious holiday
      When I create a holiday with:
        | holiday_date | 2026-04-10                           |
        | holiday_name_en | Eid Al-Fitr                     |
        | holiday_name_fr | Aïd El-Fitr                     |
        | holiday_name_ar | عيد الفطر                         |
        | holiday_type | Religious                           |
        | is_banking_holiday | true                           |
      Then the holiday should be saved successfully

    Scenario: Check if date is a banking holiday
      Given I have created a banking holiday on "2026-03-20"
      When I check if "2026-03-20" is a banking holiday
      Then the system should return true
      When I check if "2026-03-21" is a banking holiday
      Then the system should return false

    Scenario: Find banking holidays in date range
      Given I have created banking holidays on:
        | 2026-03-20 |
        | 2026-04-10 |
        | 2026-05-01 |
      When I query for banking holidays between "2026-04-01" and "2026-04-30"
      Then I should receive 1 holiday
      And the holiday should be on "2026-04-10"

  # --- System Parameter Feature ---
  Feature: System Parameter Management

    Scenario: Create an integer parameter
      When I create a system parameter with:
        | key | MAX_DAILY_TRANSFER |
        | value | 50000000           |
        | parameter_type | Integer        |
        | category | Limits             |
        | description | Max daily transfer |
        | is_active | true               |
      Then the system parameter should be saved successfully
      And I should be able to retrieve it by key "MAX_DAILY_TRANSFER"
      And the value should be "50000000"

    Scenario: Create a boolean parameter
      When I create a system parameter with:
        | key | ENABLE_REAL_TIME_NOTIFICATIONS |
        | value | true                            |
        | parameter_type | Boolean                |
        | category | Features                      |
        | description | Enable real-time alerts       |
        | is_active | true                         |
      Then the system parameter should be saved successfully

    Scenario: Create a decimal parameter
      When I create a system parameter with:
        | key | OVERDRAFT_INTEREST_RATE |
        | value | 12.5                    |
        | parameter_type | Decimal         |
        | category | Rates               |
        | description | Overdraft fee rate |
        | is_active | true                |
      Then the system parameter should be saved successfully

    Scenario: Validate integer parameter type
      When I attempt to create a system parameter with:
        | key | MAX_AMOUNT |
        | value | not_an_integer |
        | parameter_type | Integer |
      Then the system should reject it with validation error

    Scenario: List parameters by category
      Given I have created 5 parameters in "Limits" category
      And I have created 3 parameters in "Rates" category
      When I query for parameters in "Limits" category
      Then I should receive 5 parameters
      And all returned parameters should have category = "Limits"

  # --- Regulatory Code Feature ---
  Feature: Regulatory Code Management

    Scenario: Create a BCT standard risk code
      When I create a regulatory code with:
        | code | BCT001                              |
        | description_en | Standard Risk Asset        |
        | description_fr | Actif à Risque Standard    |
        | classification | StandardRisk               |
        | is_active | true                         |
      Then the regulatory code should be saved successfully

    Scenario: Create an IFRS amortized cost code
      When I create a regulatory code with:
        | code | IFRS_AC                            |
        | description_en | Amortized Cost             |
        | description_fr | Coût Amorti                |
        | classification | AmortizedCost              |
        | is_active | true                         |
      Then the regulatory code should be saved successfully

    Scenario: List all regulatory classifications
      Given I have created regulatory codes for:
        | StandardRisk |
        | LowerRisk |
        | HigherRisk |
        | AmortizedCost |
        | FairValueThroughOci |
        | FairValueThroughPl |
      When I list active regulatory codes
      Then I should receive 6 codes

  # --- Fee Schedule Feature ---
  Feature: Fee Schedule Reference Management

    Scenario: Create a transaction fee
      Given I have a currency "TND"
      When I create a fee schedule with:
        | fee_type | Transaction              |
        | amount_cents | 5000                 |
        | currency_code | TND                 |
        | description_en | Transaction fee     |
        | description_fr | Frais de transaction |
        | is_active | true                    |
      Then the fee schedule should be saved successfully

    Scenario: Create a transfer fee
      Given I have a currency "EUR"
      When I create a fee schedule with:
        | fee_type | Transfer              |
        | amount_cents | 10000             |
        | currency_code | EUR               |
        | description_en | Transfer fee      |
        | description_fr | Frais de virement |
        | is_active | true                  |
      Then the fee schedule should be saved successfully

    Scenario: Validate negative fee amount
      When I attempt to create a fee schedule with amount_cents = -5000
      Then the system should reject it with validation error
      And the error message should indicate "Fee amount cannot be negative"

    Scenario: Find all fees for a specific type
      Given I have created multiple fees for "Transaction" type in different currencies
      When I query for fees of type "Transaction"
      Then all returned fees should have fee_type = "Transaction"

    Scenario: List active fee schedules
      Given I have created 10 active fee schedules
      When I list active fee schedules
      Then I should receive 10 fee schedules
      And all returned fees should have is_active = true

  # --- Effective Date Ranges Feature ---
  Feature: Effective Date Range Management

    Scenario: Set effective date range for a country code
      When I create a country code with:
        | iso_alpha2 | XX                           |
        | iso_alpha3 | XXX                          |
        | iso_numeric| 999                          |
        | name_en | Test Country                    |
        | name_fr | Pays Test                       |
        | name_ar | دولة الاختبار                  |
        | effective_from | 2026-04-07               |
        | effective_to | 2026-12-31                  |
      Then the country should have effective_from = "2026-04-07"
      And the country should have effective_to = "2026-12-31"

    Scenario: Validate effective_to is after effective_from
      When I attempt to create a reference data with:
        | effective_from | 2026-12-31 |
        | effective_to | 2026-04-07   |
      Then the system should reject it with validation error
      And the error message should indicate "Effective end date must be after start date"

  # --- Data Consistency Feature ---
  Feature: Reference Data Consistency

    Scenario: Foreign key constraint on branch codes
      When I attempt to create a branch code referencing a non-existent bank BIC
      Then the system should reject it

    Scenario: Cascade on updates
      Given I have created a bank code "TESTBIC"
      And I have created 3 branch codes for "TESTBIC"
      When I update the bank code name
      Then the associated branch codes should remain intact
