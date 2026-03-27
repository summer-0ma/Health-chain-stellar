# CI/CD Policy and Validation Rules

## Overview

This document outlines what CI/CD validates and what it does NOT validate for pull requests.

## Current CI/CD Workflows

### 1. Dependency Security Audit (`dependency-audit.yml`)

**Purpose**: Scan dependencies for security vulnerabilities

**Validates**:
- ✅ npm audit (critical + high vulnerabilities)
- ✅ OSS Index scan (Sonatype)
- ✅ Enforces security policy thresholds

**Does NOT Validate**:
- ❌ Migration files
- ❌ Entity changes
- ❌ Schema changes
- ❌ Database structure

**Failure Criteria**:
- Critical or high severity vulnerabilities found

### 2. Contract Testing (`contract-tests.yml`)

**Purpose**: Validate module boundaries and API contracts

**Validates**:
- ✅ Module interaction contracts (BloodRequests ↔ Inventory, etc.)
- ✅ Response schema stability
- ✅ Breaking change detection
- ✅ Protected route authentication
- ✅ Code quality (ESLint, Prettier, Jest)

**Does NOT Validate**:
- ❌ Migration files
- ❌ Entity changes
- ❌ Schema changes
- ❌ Database structure

**Failure Criteria**:
- Contract test failures
- Schema breaking changes
- Module boundary violations

### 3. Smoke Tests (`smoke-tests.yml`)

**Purpose**: End-to-end integration testing

**Validates**:
- ✅ Application startup
- ✅ Basic API functionality
- ✅ Database connectivity
- ✅ Redis connectivity

**Does NOT Validate**:
- ❌ Migration files
- ❌ Entity changes
- ❌ Schema changes

**Failure Criteria**:
- Application fails to start
- API endpoints return errors
- Database/Redis connection fails

## What CI/CD Does NOT Check

### ❌ Migration Files

**Status**: NOT VALIDATED

Migration files are **NOT** required for pull requests. CI/CD does not:
- Check if migration files exist
- Validate migration file syntax
- Enforce migration file creation for schema changes
- Block PRs due to missing migrations

**Rationale**:
- Development uses automatic schema synchronization
- Migrations are created before production deployment
- Separates development workflow from deployment workflow

### ❌ Entity Changes

**Status**: NOT VALIDATED

Entity file changes are **NOT** validated by CI/CD. However:
- Entity changes are detected by contract tests (schema snapshots)
- Breaking changes to response schemas ARE detected
- Entity syntax is validated by TypeScript compiler

### ❌ Database Schema Changes

**Status**: NOT VALIDATED

Database schema changes are **NOT** directly validated. However:
- Response schema changes ARE detected by contract tests
- Breaking API changes ARE detected
- Schema synchronization works in development

## PR Merge Requirements

### Required Checks (Must Pass)

1. ✅ **Dependency Security Audit** - No critical/high vulnerabilities
2. ✅ **Contract Tests** - All tests pass, no breaking changes
3. ✅ **Smoke Tests** - Application starts and basic APIs work

### Optional Checks (Nice to Have)

- Code review approval
- Test coverage improvements
- Documentation updates

### NOT Required

- ❌ Migration files
- ❌ Entity file changes (if no API breaking changes)
- ❌ Database schema validation

## Development Workflow

### For Feature Development

```bash
# 1. Create feature branch
git checkout -b feat/new-feature

# 2. Modify entities (if needed)
vim src/users/entities/user.entity.ts

# 3. TypeORM auto-syncs schema in development
npm run start:dev

# 4. Write/update tests
npm test

# 5. Commit changes
git add .
git commit -m "feat: add new feature"

# 6. Push and create PR
git push origin feat/new-feature
```

**CI/CD will check**:
- ✅ Code quality
- ✅ Tests pass
- ✅ No breaking API changes
- ✅ No security vulnerabilities

**CI/CD will NOT check**:
- ❌ Migration files (not required)
- ❌ Entity changes (unless they break API)

### For Production Deployment

```bash
# 1. Create migration (if schema changed)
npm run typeorm migration:generate -- -n DescribeChange

# 2. Test migration in staging
npm run typeorm migration:run

# 3. Include migration in deployment PR
git add src/migrations/
git commit -m "feat: add migration for schema change"

# 4. Deploy with migration
npm run typeorm migration:run  # Run before starting app
npm run start:prod
```

## Bypassing CI/CD Checks

### When to Bypass

- Emergency hotfixes (use with caution)
- Known failing tests (with plan to fix)
- Temporary debugging

### How to Bypass

```bash
# Bypass all checks (not recommended)
git push --force-with-lease

# Bypass pre-commit hooks
git commit --no-verify
```

**⚠️ WARNING**: Bypassing checks should be rare and documented.

## Troubleshooting CI/CD Failures

### Dependency Audit Failed

```bash
# Check vulnerabilities
npm audit

# Fix vulnerabilities
npm audit fix

# Or apply exception (see SECURITY_POLICY.md)
```

### Contract Tests Failed

```bash
# Run tests locally
npm run test:contracts

# Review breaking changes
npm run test:contracts -- --verbose

# Update snapshots if intentional
npm run test:contracts -- -u
```

### Smoke Tests Failed

```bash
# Check application startup
npm run start:dev

# Check database connection
npm run typeorm migration:show

# Check Redis connection
redis-cli ping
```

## Future Enhancements

Potential CI/CD improvements:

- [ ] Performance regression testing
- [ ] Load testing
- [ ] Database migration validation (optional)
- [ ] API documentation generation
- [ ] Automated changelog generation

## References

- [Dependency Security Audit](../.github/workflows/dependency-audit.yml)
- [Contract Testing](../.github/workflows/contract-tests.yml)
- [Smoke Tests](../.github/workflows/smoke-tests.yml)
- [Migration Policy](./MIGRATION_POLICY.md)
- [Security Policy](../SECURITY_POLICY.md)

## Related Issues

- #256: Remove migration linter/check in CI
- #243: API versioning strategy
