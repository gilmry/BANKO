# Architecture BANKO

## Vue d'ensemble

BANKO suit une architecture **hexagonale** (Ports & Adaptateurs) avec **Domain-Driven Design** (DDD).

### Diagramme systeme global

```mermaid
graph TB
    subgraph Frontend
        ASTRO[Astro + Svelte]
    end

    subgraph API_Gateway
        TRAEFIK[Traefik]
    end

    subgraph Backend["Backend Rust (Hexagonal)"]
        subgraph Infrastructure
            HANDLERS[HTTP Handlers]
            REPOS[PostgreSQL Repos]
            CACHE[Redis Cache]
        end
        subgraph Application
            UC[Use Cases]
            PORTS[Ports / Traits]
            DTO[DTOs]
        end
        subgraph Domain
            ENT[Entities]
            VO[Value Objects]
            SVC[Domain Services]
        end
    end

    subgraph Data
        PG[(PostgreSQL 16)]
        MINIO[(MinIO S3)]
    end

    subgraph Monitoring
        PROM[Prometheus]
        GRAF[Grafana]
    end

    ASTRO --> TRAEFIK
    TRAEFIK --> HANDLERS
    HANDLERS --> UC
    UC --> PORTS
    PORTS --> ENT
    PORTS --> VO
    REPOS -.implements.-> PORTS
    REPOS --> PG
    HANDLERS --> MINIO
    HANDLERS --> PROM
    PROM --> GRAF
```

### 12 Bounded Contexts

```mermaid
graph LR
    subgraph Core
        CUST[Customer BC1]
        ACCT[Account BC2]
        CRED[Credit BC3]
        PAY[Payment BC9]
    end

    subgraph Compliance
        AML[AML BC4]
        SANC[Sanctions BC5]
        PRUD[Prudential BC6]
    end

    subgraph Operations
        ACCTG[Accounting BC7]
        RPT[Reporting BC8]
        FX[FX BC10]
    end

    subgraph Platform
        GOV[Governance BC11]
        ID[Identity BC12]
    end

    CUST --> ACCT
    ACCT --> PAY
    CUST --> AML
    CUST --> SANC
    ACCT --> PRUD
    CRED --> PRUD
    ACCT --> ACCTG
    ACCTG --> RPT
    PAY --> FX
    ID --> GOV
```

### Architecture hexagonale par crate

```mermaid
graph TB
    subgraph "crates/infrastructure"
        WEB[web/handlers]
        DB[database/repositories]
        CFG[config]
    end

    subgraph "crates/application"
        UC2[use_cases]
        P[ports]
        D[dto]
    end

    subgraph "crates/domain"
        E[entities]
        V[value_objects]
        S[services]
        ERR[errors]
    end

    WEB --> UC2
    WEB --> D
    DB -.implements.-> P
    UC2 --> P
    UC2 --> E
    UC2 --> V
    P --> E
    E --> V
    E --> ERR
    V --> ERR
```

## Stack Docker Compose

```mermaid
graph LR
    subgraph Docker["Docker Compose"]
        T[Traefik :80]
        API[API :8080]
        FE[Frontend :3000]
        PG2[PostgreSQL :5432]
        MIN[MinIO :9000]
        PR[Prometheus :9090]
        GR[Grafana :3001]
    end

    T --> API
    T --> FE
    API --> PG2
    API --> MIN
    PR --> API
    GR --> PR
```

## Workspace Cargo

| Crate | Dependencies externes | Role |
|-------|----------------------|------|
| `banko-domain` | serde, thiserror, uuid, chrono | Logique metier pure |
| `banko-application` | domain + async-trait | Use cases, ports |
| `banko-infrastructure` | application + domain + actix-web + sqlx + tokio + tracing | Adaptateurs |
