Feature: Islamic Banking (P1-BC2)
  As a Sharia-compliant banking officer
  I want to manage Islamic financing products and ensure Sharia compliance
  So that the bank can offer compliant Islamic financial services

  Background:
    Given the system is initialized
    And I am authenticated as "islamic_banking_officer"
    And Islamic banking module is operational
    And Sharia Board is constituted

  # Murabaha (Cost-Plus) Contracts
  @critical @bc-islamic-001 @murabaha
  Scenario: Create Murabaha contract for vehicle purchase
    Given a customer with id "cust-ib-001" and name "Fatima Al-Rashid"
    And Sharia Board approval obtained on "2026-04-07"
    When I create Murabaha contract for vehicle purchase:
      | Asset Description | Toyota Corolla 2026 |
      | Asset Cost Price | 45000 TND |
      | Murabaha Markup | 5000 TND (11.11%) |
      | Total Price | 50000 TND |
      | Installments | 24 months |
      | Monthly Payment | 2083.33 TND |
    Then Murabaha contract is created with code "MUR-2026-001"
    And asset ownership transfers to customer after full payment
    And late payment penalty is specified as deferred

  @high @bc-islamic-002 @murabaha
  Scenario: Process Murabaha installment payment
    Given active Murabaha contract "MUR-2026-001" with 24 instalments
    When customer pays first instalment "2083.33 TND" on "2026-05-07"
    Then payment is recorded as:
      | Payment Date | 2026-05-07 |
      | Amount | 2083.33 TND |
      | Remaining Instalments | 23 |
      | Next Due Date | 2026-06-07 |
    And customer receives payment receipt

  @high @bc-islamic-003 @murabaha
  Scenario: Handle Murabaha payment default
    Given Murabaha contract with due payment "2083.33 TND"
    When payment is not received by due date + 30 days
    Then late payment detected
    And system applies deferral penalty (collected for charity)
    And customer is sent reminder notice
    And if payment continues to delay, escalation occurs

  @critical @bc-islamic-004 @murabaha
  Scenario: Complete Murabaha contract
    Given Murabaha contract "MUR-2026-001" with 23 remaining instalments
    When final instalment is paid on "2028-04-07"
    Then contract status changes to "completed"
    And asset ownership transfers to customer
    And completion certificate is issued
    And transaction is recorded in Islamic banking reports

  # Ijara (Lease) Contracts
  @critical @bc-islamic-005 @ijara
  Scenario: Create Ijara Muntahia Bittamleek (lease with ownership transfer)
    Given a customer with id "cust-ib-002"
    And property valued at "300000 TND"
    When I create Ijara contract:
      | Lease Type | Ijara Muntahia Bittamleek |
      | Asset Description | Commercial space in Tunis CBD |
      | Lease Term | 15 years |
      | Annual Rental | 20000 TND |
      | Ownership Transfer Date | End of Year 15 |
      | Residual Value | 50000 TND |
    Then Ijara contract is registered with code "IJR-2026-001"
    And rental payments are scheduled monthly: "1666.67 TND"
    And ownership transfer is recorded for end of lease period

  @high @bc-islamic-006 @ijara
  Scenario: Process Ijara rental payment
    Given active Ijara contract "IJR-2026-001" with annual rental "20000 TND"
    When monthly rental "1666.67 TND" is due on "2026-05-07"
    And customer pays rental
    Then payment is recorded as Ijara rental income
    And next payment schedule is displayed
    And balance due is updated

  @high @bc-islamic-007 @ijara
  Scenario: Handle Ijara lease early termination
    Given Ijara contract "IJR-2026-001" with remaining term "10 years"
    When lessee requests early termination
    Then system calculates lease breakage cost:
      | Remaining Rentals Present Value | Calculated |
      | Lessee Compensation (if required) | Calculated |
    And termination is approved and documented
    And asset is returned or repossessed
    And settlement is processed

  @critical @bc-islamic-008 @ijara
  Scenario: Complete Ijara contract with ownership transfer
    Given Ijara contract "IJR-2026-001" approaching end of lease term
    When final rental payment is made on "2041-04-07"
    Then contract status changes to "completed"
    And ownership is officially transferred to lessee
    And ownership transfer document is executed
    And property record is updated
    And transaction completion certificate is issued

  # Musharaka (Profit Sharing Partnership)
  @critical @bc-islamic-009 @musharaka
  Scenario: Create Permanent Musharaka partnership
    Given a customer with id "cust-ib-003" for business project
    And business plan approved by bank
    When I create Permanent Musharaka contract:
      | Project Name | Textile Manufacturing Facility |
      | Bank Capital Contribution | 500000 TND |
      | Customer Capital Contribution | 300000 TND |
      | Total Capital | 800000 TND |
      | Bank Profit Share | 62.5% |
      | Customer Profit Share | 37.5% |
      | Contract Duration | 7 years |
      | Profit Distribution | Quarterly |
    Then Musharaka contract is registered with code "MUSH-2026-001"
    And both partners execute partnership agreement
    And governance structure is defined (management roles)

  @high @bc-islamic-010 @musharaka
  Scenario: Calculate and distribute Musharaka quarterly profits
    Given active Musharaka contract "MUSH-2026-001"
    When quarterly profit calculation occurs for Q1 2026:
      | Gross Revenue | 450000 TND |
      | Operating Costs | 250000 TND |
      | Quarterly Profit | 200000 TND |
    Then profit is distributed:
      | Bank Share (62.5%) | 125000 TND |
      | Customer Share (37.5%) | 75000 TND |
    And both parties receive distribution statements
    And distributions are paid to respective accounts

  @high @bc-islamic-011 @musharaka
  Scenario: Handle Musharaka loss scenario
    Given Musharaka contract with expected profitability
    When Q2 results show operational loss:
      | Gross Revenue | 200000 TND |
      | Operating Costs | 250000 TND |
      | Quarterly Loss | -50000 TND |
    Then loss is allocated pro-rata:
      | Bank Loss (62.5%) | -31250 TND |
      | Customer Loss (37.5%) | -18750 TND |
    And both parties document loss
    And remediation discussion is scheduled

  @critical @bc-islamic-012 @musharaka
  Scenario: Complete Declining Musharaka with bank exit
    Given Declining Musharaka contract "MUSH-2026-001" with 20% annual reduction
    When Year 7 (final year) concludes:
      | Bank's Remaining Stake | 12.5% |
      | Customer's Final Buyout Amount | Calculated |
    Then bank exits the partnership
    And full settlement is processed
    And partnership documentation is archived
    And final audited statement is prepared

  # Mudaraba (Investment Agency)
  @critical @bc-islamic-013 @mudaraba
  Scenario: Create Restricted Mudaraba for specific project
    Given a customer (Rabb Al-Mal) with investment capital "200000 TND"
    When I create Restricted Mudaraba contract:
      | Investment Type | Restricted Mudaraba |
      | Project Name | Real Estate Development |
      | Investment Capital | 200000 TND |
      | Bank Management Fee | 2% annually |
      | Profit Share (Customer) | 70% |
      | Profit Share (Bank) | 30% |
      | Investment Period | 3 years |
      | Investment Restrictions | Real estate only, Tunis region |
    Then Mudaraba contract is registered with code "MUDA-2026-001"
    And fund transfer is initiated to bank's designated account
    And investment begins per restrictions

  @high @bc-islamic-014 @mudaraba
  Scenario: Distribute Mudaraba profits to investor
    Given active Mudaraba contract "MUDA-2026-001"
    When annual profit calculation shows:
      | Investment Capital | 200000 TND |
      | Annual Profit | 40000 TND |
      | Bank Management Fee (2%) | 4000 TND |
      | Distributable Profit | 36000 TND |
    Then profit is allocated:
      | Investor Share (70%) | 25200 TND |
      | Bank Share (30%) | 10800 TND |
    And distribution statement is sent to investor
    And funds are credited to investor account

  @high @bc-islamic-015 @mudaraba
  Scenario: Handle Mudaraba capital loss
    Given Mudaraba investment with capital "200000 TND"
    When market downturn results in capital loss:
      | Investment Value | 180000 TND |
      | Capital Loss | -20000 TND |
    Then loss is borne entirely by investor (capital provider)
    And bank's management fee is still charged
    And revised investment statement is provided
    And investor is informed of options (continuation/withdrawal)

  @critical @bc-islamic-016 @mudaraba
  Scenario: Complete Mudaraba investment period
    Given Mudaraba contract "MUDA-2026-001" with 3-year term ending "2029-04-07"
    When investment period concludes:
      | Final Investment Value | 280000 TND |
      | Total Profit Over 3 Years | 80000 TND |
    Then final settlement is calculated
    And investor receives capital + final distribution
    And bank receives final fees and profit share
    And investment is officially closed
    And audited statement is prepared

  # Sukuk (Islamic Bonds)
  @critical @bc-islamic-017 @sukuk
  Scenario: Issue Murabaha Sukuk
    Given bank needs to raise capital "5000000 TND"
    When Sukuk issuance is planned:
      | Sukuk Type | Murabaha Sukuk |
      | Face Value | 1000 TND per unit |
      | Total Units | 5000 |
      | Coupon Rate | 5.5% p.a. |
      | Maturity | 5 years |
      | Rating | A+ (Sharia-compliant) |
      | Underlying Assets | Trade receivables portfolio |
    Then Sukuk prospectus is prepared
    And Sharia Board issues compliance certificate
    And Sukuk is offered to investors
    And funds are received by bank

  @high @bc-islamic-018 @sukuk
  Scenario: Make periodic Sukuk coupon payment
    Given issued Murabaha Sukuk with 5.5% annual coupon
    When semi-annual coupon payment is due:
      | Total Outstanding Units | 4950 (50 redeemed) |
      | Coupon Rate | 5.5% p.a. / 2 = 2.75% per period |
      | Coupon Payment | 135,862.5 TND |
      | Payment Date | 2026-10-07 |
    Then coupon is paid to Sukuk holders
    And payment record is created
    And holder accounts are credited

  @high @bc-islamic-019 @sukuk
  Scenario: Redeem Sukuk at maturity
    Given Sukuk approaching maturity on "2031-04-07"
    When maturity date arrives:
      | Outstanding Sukuk Units | 4000 |
      | Face Value per Unit | 1000 TND |
      | Redemption Amount | 4000000 TND |
      | Final Coupon (if applicable) | Included |
    Then redemption is processed
    And holders receive face value + final coupon
    And Sukuk is delisted from Islamic capital markets
    And issuance is closed

  # Zakat Calculations
  @critical @bc-islamic-020 @zakat
  Scenario: Calculate Zakat for Islamic banking customer
    Given customer with Islamic banking account
    When annual Zakat calculation occurs on "2026-04-07":
      | Total Assets | 500000 TND |
      | Total Liabilities | 150000 TND |
      | Zakatable Wealth | 350000 TND |
      | Zakat Rate | 2.5% |
      | Zakat Amount Due | 8750 TND |
    Then Zakat calculation is recorded with code "ZAKAT-2026-001"
    And customer is notified of Zakat obligation
    And customer can elect to pay directly or via bank

  @high @bc-islamic-021 @zakat
  Scenario: Distribute Zakat funds to charities
    Given Zakat amount due "8750 TND"
    When customer authorizes bank to distribute to qualified charities:
      | Charity 1 | Orphanage | 2500 TND |
      | Charity 2 | Medical Clinic | 3000 TND |
      | Charity 3 | Education Fund | 3250 TND |
    Then Zakat is distributed per customer's instructions
    And distribution receipts are provided
    And Zakat certificate is issued
    And transaction is recorded in Islamic banking reports

  @high @bc-islamic-022 @zakat
  Scenario: Issue Zakat compliance certificate
    Given Zakat paid in full "8750 TND"
    When certificate is requested
    Then Zakat compliance certificate is issued with:
      | Amount Paid | 8750 TND |
      | Payment Date | 2026-04-15 |
      | Distribution Details | Listed |
      | Sharia Scholar Endorsement | Yes |
    And certificate is Sharia Board approved

  # Sharia Board Decisions
  @critical @bc-islamic-023 @sharia
  Scenario: Sharia Board approves new Islamic product
    Given proposed new Islamic finance product "Salam Financing"
    When Sharia Board meets on "2026-04-07":
      | Board Members | 5 (unanimous) |
      | Decision | Approve product |
      | Conditions | Insurance must be Sharia-compliant |
    Then decision code is "DECISION-2026-001"
    And decision is documented with detailed reasoning
    And implementation requirements are communicated
    And product can proceed to development

  @high @bc-islamic-024 @sharia
  Scenario: Sharia Board interprets contract ambiguity
    Given Murabaha contract interpretation question from operations team
    When Sharia Board reviews on "2026-04-10":
      | Question | Whether markup can be deferred |
      | Interpretation | Markup must be disclosed upfront and paid |
      | Status | Active ruling |
    Then board fatwa is issued
    And all Murabaha officers are trained on interpretation
    And affected contracts are reviewed for compliance

  @high @bc-islamic-025 @sharia
  Scenario: Sharia Board investigates compliance violation
    Given potential Sharia compliance violation detected
    When Sharia Board initiates investigation:
      | Violation Type | Interest-bearing arrangement in Murabaha |
      | Investigation Date | 2026-04-07 |
    Then board conducts thorough investigation
    And violations are documented
    And remediation plan is agreed
    And corrective actions are monitored

  # Profit Distribution and Reporting
  @high @bc-islamic-026 @distribution
  Scenario: Generate quarterly Islamic banking profit statement
    Given all Islamic contracts for Q1 2026
    When profit distribution reporting occurs:
      | Total Murabaha Profits | 125000 TND |
      | Musharaka Distributions | 75000 TND |
      | Mudaraba Bank Share | 45000 TND |
      | Ijara Income | 60000 TND |
      | Sukuk Coupon Paid | 135862.5 TND |
    Then consolidated report is generated
    And report is Sharia Board reviewed
    And report is submitted to regulators (BCT)

  @high @bc-islamic-027 @distribution
  Scenario: Monitor Islamic banking product performance
    Given Islamic products portfolio for 2026
    When quarterly performance analysis occurs:
      | Murabaha Volume | 2500000 TND |
      | Ijara Portfolio | 4000000 TND |
      | Musharaka Partnerships | 2000000 TND |
      | Sukuk Issuance | 5000000 TND |
      | Total Islamic Assets | 13500000 TND |
      | Islamic Banking Market Share | 15% |
    Then performance metrics are captured
    And profitability analysis is completed
    And trends are reported to management

  @critical @bc-islamic-028 @compliance
  Scenario: Ensure Sharia compliance in all transactions
    Given all Islamic banking transactions for a period
    When compliance verification occurs:
      | Total Contracts Reviewed | 500 |
      | Sharia Compliant | 495 (99%) |
      | Non-Compliant or Issue | 5 (1%) |
      | Remediation Required | Yes |
    Then non-compliant contracts are identified
    And customer communications are sent
    And corrective actions are documented
    And follow-up verification is scheduled

  @high @bc-islamic-029 @reporting
  Scenario: Regulatory filing for Islamic banking operations
    Given all Islamic banking operations for H1 2026
    When filing to BCT (Banque Centrale de Tunisie) is prepared:
     