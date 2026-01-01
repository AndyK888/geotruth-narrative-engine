# Security Guidelines

This document outlines security practices for developing, deploying, and operating GeoTruth.

---

## 🔐 Core Security Principles

1. **Privacy by Design**: Video files never leave the user's device
2. **Minimal Data Collection**: Only anonymized GPS coordinates are sent to the cloud
3. **Secure Storage**: Secrets stored in OS-native keychains
4. **Defense in Depth**: Multiple layers of security at each tier

---

## 🔑 Authentication & Authorization

### JWT-Based Authentication

All API requests require a valid JWT token:

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
```

**Token Configuration:**
```python
# backend/app/config.py

class Settings:
    jwt_secret: str          # Minimum 256 bits
    jwt_algorithm: str = "HS256"
    jwt_expire_minutes: int = 1440  # 24 hours
```

### Token Refresh

- Access tokens expire after 24 hours
- Refresh tokens valid for 30 days
- Refresh tokens are single-use (rotated on each refresh)

### Rate Limiting

Prevent abuse with tiered rate limits:

| Tier | Rate Limit | Description |
|------|------------|-------------|
| Free | 100 req/min | Basic usage |
| Pro | 1000 req/min | Power users |
| Enterprise | Unlimited | Custom limits |

---

## 🗄️ Secret Management

### Desktop Application

**Never store secrets in:**
- Configuration files
- Local storage
- Plain text anywhere

**DO use OS Keychain:**

```rust
// src-tauri/src/services/keychain.rs

use keyring::Entry;

const SERVICE_NAME: &str = "com.geotruth.desktop";

pub fn store_secret(key: &str, value: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.set_password(value)?;
    Ok(())
}

pub fn get_secret(key: &str) -> Result<String, keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.get_password()
}

pub fn delete_secret(key: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.delete_password()?;
    Ok(())
}
```

**Stored secrets:**
- JWT authentication token
- User's BYOK Gemini API key (optional)

### Backend Services

**Environment-based configuration:**

```bash
# .env (never commit this file!)
JWT_SECRET=your-256-bit-secret-key-here
GEMINI_API_KEY=AIza...
POSTGRES_PASSWORD=strong-database-password
```

**Docker secret injection:**

```yaml
# docker-compose.prod.yml
services:
  api:
    environment:
      - JWT_SECRET_FILE=/run/secrets/jwt_secret
    secrets:
      - jwt_secret

secrets:
  jwt_secret:
    file: ./secrets/jwt_secret.txt
```

---

## 🔒 Data Protection

### Data Classification

| Category | Examples | Protection |
|----------|----------|------------|
| **Public** | POI names, facts | None required |
| **Internal** | Aggregated analytics | Access controls |
| **Confidential** | User GPS tracks | Encrypted, access-logged |
| **Restricted** | API keys, passwords | Encrypted, keychain only |

### Privacy Boundaries

```
┌─────────────────────────────────────────────────────────────┐
│                    USER'S DEVICE                             │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                 NEVER LEAVES DEVICE                   │   │
│  │  • Raw video files                                    │   │
│  │  • Audio tracks                                       │   │
│  │  • Extracted frames                                   │   │
│  │  • Thumbnail images                                   │   │
│  │  • Full transcripts (stored locally)                  │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTPS (TLS 1.3)
                            │ Anonymized data only
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    CLOUD SERVICES                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │               RECEIVED FROM CLIENT                    │   │
│  │  • GPS coordinates (anonymized, no user ID linked)    │   │
│  │  • Transcript for narration (optional)                │   │
│  │  • Event metadata                                     │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Data Encryption

**At Rest:**
- DuckDB databases: Encrypted with user's OS keychain master key
- Backups: AES-256 encrypted before upload

**In Transit:**
- All API calls: TLS 1.3 minimum
- Certificate pinning: Enabled for production

---

## 🌐 API Security

### Content Security Policy

```json
// tauri.conf.json
{
  "app": {
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://tile.openstreetmap.org; connect-src 'self' https://api.geotruth.io"
    }
  }
}
```

### Input Validation

All inputs are validated with Pydantic:

```python
from pydantic import BaseModel, Field, validator

class EnrichRequest(BaseModel):
    lat: float = Field(..., ge=-90, le=90)
    lon: float = Field(..., ge=-180, le=180)
    timestamp: Optional[str] = None
    
    @validator('timestamp')
    def validate_timestamp(cls, v):
        if v is not None:
            try:
                datetime.fromisoformat(v.replace('Z', '+00:00'))
            except ValueError:
                raise ValueError('Invalid ISO 8601 timestamp')
        return v
```

### SQL Injection Prevention

Always use parameterized queries:

```python
# ❌ NEVER DO THIS
query = f"SELECT * FROM pois WHERE name = '{user_input}'"

# ✅ ALWAYS DO THIS
query = "SELECT * FROM pois WHERE name = $1"
result = await conn.fetch(query, user_input)
```

### Request Logging

Log all requests for audit:

```python
import structlog

logger = structlog.get_logger()

@app.middleware("http")
async def log_requests(request: Request, call_next):
    request_id = str(uuid4())
    
    logger.info(
        "request_started",
        request_id=request_id,
        method=request.method,
        path=request.url.path,
        client_ip=request.client.host
    )
    
    response = await call_next(request)
    
    logger.info(
        "request_completed",
        request_id=request_id,
        status_code=response.status_code
    )
    
    return response
```

---

## 🐳 Container Security

### Dockerfile Best Practices

```dockerfile
# Use specific version, not :latest
FROM python:3.11-slim as builder

# Run as non-root user
RUN useradd -m -u 1000 appuser

# Don't run as root
USER appuser

# Use read-only filesystem where possible
# Use --mount for secrets
RUN --mount=type=secret,id=pip_config \
    pip install --no-cache-dir -r requirements.txt
```

### Docker Compose Production

```yaml
# docker-compose.prod.yml
services:
  api:
    read_only: true
    security_opt:
      - no-new-privileges:true
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE
    tmpfs:
      - /tmp:noexec,nosuid,nodev,size=100m
```

### Network Isolation

```yaml
services:
  api:
    networks:
      - frontend
      - backend
  
  geo-db:
    networks:
      - backend  # Only accessible to api

networks:
  frontend:
  backend:
    internal: true  # No external access
```

---

## 🔍 Vulnerability Management

### Dependency Scanning

```yaml
# .github/workflows/security.yml
name: Security Scan

on:
  push:
    branches: [main, develop]
  schedule:
    - cron: '0 0 * * *'  # Daily

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      # Node dependencies
      - run: pnpm audit
      
      # Rust dependencies
      - run: cargo audit
      
      # Python dependencies
      - run: pip-audit -r requirements.txt
      
      # Container scanning
      - uses: aquasecurity/trivy-action@master
        with:
          image-ref: 'geotruth/api:latest'
```

### Update Policy

| Component | Update Frequency | Automation |
|-----------|------------------|------------|
| OS Packages | Weekly | Dependabot |
| Node Deps | Weekly | Dependabot |
| Rust Deps | Weekly | Dependabot |
| Python Deps | Weekly | Dependabot |
| Base Images | Monthly | Manual review |

---

## 🚨 Incident Response

### Security Issue Reporting

**For external researchers:**
- Email: security@geotruth.io
- PGP Key: Available on website
- Response time: 48 hours

**For team members:**
1. Create private GitHub issue
2. Label: `security`
3. Notify team lead immediately

### Incident Severity Levels

| Level | Description | Response Time | Examples |
|-------|-------------|---------------|----------|
| P0 | Critical | 1 hour | Data breach, RCE |
| P1 | High | 4 hours | Auth bypass, SQL injection |
| P2 | Medium | 24 hours | XSS, CSRF |
| P3 | Low | 1 week | Info disclosure, minor bugs |

### Response Checklist

1. **Contain**: Isolate affected systems
2. **Assess**: Determine scope and impact
3. **Remediate**: Apply fixes or patches
4. **Notify**: Inform affected users if required
5. **Document**: Create postmortem

---

## ✅ Security Checklist

### Before Release

- [ ] All dependencies updated
- [ ] Security scan passed (Snyk/Trivy)
- [ ] No secrets in code or configs
- [ ] API endpoints authenticated
- [ ] Rate limiting configured
- [ ] CORS properly configured
- [ ] CSP headers set
- [ ] Logging enabled (without sensitive data)

### Ongoing

- [ ] Weekly dependency audits
- [ ] Monthly access review
- [ ] Quarterly penetration testing
- [ ] Annual security training

---

## 📚 Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Tauri Security](https://tauri.app/v1/guides/security/)
- [FastAPI Security](https://fastapi.tiangolo.com/advanced/security/)

---

## 📞 Contact

**Security Team:**
- Email: security@geotruth.io
- Slack: #security-team
