# ALICE-IaC-SaaS

Infrastructure as Code platform — Terraform plan/apply orchestration, state management, and continuous monitoring via the ALICE SaaS architecture.

## Architecture

```
Client
  └─ API Gateway (:8141) — JWT auth, rate limiting, proxy
       └─ Core Engine (:9141) — IaC orchestration, state store, monitor
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| POST | /api/v1/iac/plan | Generate Terraform execution plan |
| POST | /api/v1/iac/apply | Apply infrastructure changes |
| GET | /api/v1/iac/state | Retrieve current infrastructure state |
| GET | /api/v1/iac/monitor | Live infrastructure health monitor |
| GET | /api/v1/iac/stats | Request statistics |

## Quick Start

```bash
cd services/core-engine && cargo run
cd services/api-gateway && cargo run
```

## License

AGPL-3.0-or-later
