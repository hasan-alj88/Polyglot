## Security Architecture

### Authentication & Authorization (Future)

**MVP:** No authentication (local development only)

**Post-MVP:**
- CLI: API key or token-based auth
- Services: mTLS for inter-service communication (if distributed)
- Database: PostgreSQL role-based access control

### Data Protection

**At Rest:**
- PostgreSQL: Enable encryption at rest (OS-level or managed database)
- Redis: Persistence encryption if enabled

**In Transit:**
- PostgreSQL: TLS connections (`sslmode=require`)
- Redis: TLS enabled (`tls-port` configuration)

### Input Validation

**CLI:**
- clap validates argument types
- Path traversal prevention for `.pg` file paths
- Sanitize user input before database queries (SQLx parameterized queries prevent SQL injection)

**Lexer/Parser:**
- Reject malformed `.pg` files
- Limit file size (prevent DoS via massive files)
- Timeout for compilation (prevent infinite loops)

### Process Isolation

**Services:**
- Each service runs as separate process
- Failures isolated (one service crash doesn't affect others)
- Database connection pooling prevents connection exhaustion

**Runtime Wrappers:**
- Future: Sandbox Python/Node/Rust execution (namespaces, cgroups, or containers)
- MVP: Subprocess isolation only

### Secrets Management

**Configuration:**
- Never hardcode credentials in code
- Database URL via environment variable `DATABASE_URL`
- Redis URL via environment variable `REDIS_URL`
- Future: Integration with HashiCorp Vault or AWS Secrets Manager

---

