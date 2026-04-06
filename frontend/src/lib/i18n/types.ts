export interface TranslationKeys {
  app: { name: string; tagline: string };
  nav: {
    home: string;
    accounts: string;
    transfers: string;
    profile: string;
    logout: string;
    settings: string;
    customers: string;
    credits: string;
    dashboard: string;
    reports: string;
    governance: string;
    accounting: string;
  };
  common: {
    save: string;
    cancel: string;
    delete: string;
    edit: string;
    loading: string;
    error: string;
    success: string;
    confirm: string;
    back: string;
    next: string;
    submit: string;
    search: string;
    filter: string;
    export: string;
    page: string;
    of: string;
    showing: string;
    results: string;
    yes: string;
    no: string;
    close: string;
    create: string;
    update: string;
    details: string;
    status: string;
    date: string;
    amount: string;
    description: string;
    actions: string;
    active: string;
    inactive: string;
    pending: string;
    approved: string;
    rejected: string;
    currency: string;
  };
  auth: {
    login: { title: string; email: string; password: string; submit: string; forgot_password: string };
    register: { title: string; first_name: string; last_name: string; email: string; password: string; confirm_password: string; submit: string };
    two_factor: { title: string; code: string; submit: string; resend: string };
    logout: string;
  };
  roles: {
    admin: string;
    manager: string;
    agent: string;
    auditor: string;
    compliance: string;
    customer: string;
  };
  customer: {
    title: string;
    create: {
      title: string;
      first_name: string;
      last_name: string;
      email: string;
      phone: string;
      address: string;
      cin: string;
      birth_date: string;
      nationality: string;
      submit: string;
    };
    kyc: {
      title: string;
      status: string;
      level: string;
      documents: string;
      verify: string;
      verified: string;
      pending: string;
      rejected: string;
      expiry_date: string;
    };
    risk: {
      title: string;
      level: string;
      low: string;
      medium: string;
      high: string;
      critical: string;
      score: string;
      last_review: string;
    };
  };
  account: {
    title: string;
    my_accounts: string;
    types: {
      current: string;
      savings: string;
      term_deposit: string;
      joint: string;
    };
    balance: {
      available: string;
      current: string;
      reserved: string;
    };
    transfer: {
      title: string;
      from: string;
      to: string;
      amount: string;
      reference: string;
      submit: string;
      success: string;
    };
    movements: {
      title: string;
      credit: string;
      debit: string;
      date: string;
      label: string;
      amount: string;
      balance_after: string;
    };
    open: string;
    close: string;
    rib: string;
    iban: string;
  };
  credit: {
    title: string;
    request: string;
    amount: string;
    duration: string;
    rate: string;
    monthly_payment: string;
    total_cost: string;
    status: string;
    types: {
      personal: string;
      mortgage: string;
      business: string;
      auto: string;
    };
    guarantees: string;
    schedule: string;
    remaining: string;
  };
  aml: {
    title: string;
    alerts: string;
    threshold: string;
    suspicious_activity: string;
    report: string;
    investigation: string;
    status: {
      open: string;
      investigating: string;
      closed: string;
      escalated: string;
    };
    scenarios: string;
    freeze: string;
  };
  sanctions: {
    title: string;
    screening: string;
    lists: string;
    match: string;
    no_match: string;
    potential_match: string;
    false_positive: string;
    last_check: string;
    source: string;
  };
  prudential: {
    title: string;
    capital_ratio: string;
    solvency: string;
    liquidity: string;
    leverage: string;
    risk_weighted_assets: string;
    tier1: string;
    tier2: string;
    requirements: string;
  };
  accounting: {
    title: string;
    journal: string;
    entries: string;
    debit: string;
    credit: string;
    balance: string;
    period: string;
    account_code: string;
    general_ledger: string;
    trial_balance: string;
    closing: string;
  };
  reporting: {
    title: string;
    generate: string;
    schedule: string;
    regulatory: string;
    statistical: string;
    custom: string;
    format: string;
    period: string;
    download: string;
  };
  governance: {
    title: string;
    roles: string;
    permissions: string;
    audit_trail: string;
    policies: string;
    committees: string;
    assign_role: string;
    revoke_role: string;
  };
  payment: {
    title: string;
    sepa: string;
    swift: string;
    domestic: string;
    international: string;
    beneficiary: string;
    bic: string;
    status: {
      initiated: string;
      processing: string;
      completed: string;
      failed: string;
      cancelled: string;
    };
  };
  forex: {
    title: string;
    exchange_rate: string;
    buy: string;
    sell: string;
    pair: string;
    spot: string;
    forward: string;
    conversion: string;
  };
  dashboard: {
    title: string;
    solvency_ratio: string;
    tier1_ratio: string;
    cd_ratio: string;
    concentration: string;
    alerts: string;
    overview: string;
    recent_activity: string;
    total_assets: string;
    total_liabilities: string;
    total_customers: string;
    total_accounts: string;
  };
  audit: {
    title: string;
    who: string;
    when: string;
    what: string;
    filter: string;
    export: string;
    action: string;
    entity: string;
    old_value: string;
    new_value: string;
  };
  validation: {
    required: string;
    email: string;
    min_length: string;
    max_length: string;
    password_uppercase: string;
    password_lowercase: string;
    password_number: string;
    password_special: string;
    password_min_length: string;
    passwords_match: string;
    cin_format: string;
    rib_format: string;
    iban_format: string;
    amount_positive: string;
    amount_max: string;
    phone_format: string;
    date_past: string;
    date_future: string;
  };
}
