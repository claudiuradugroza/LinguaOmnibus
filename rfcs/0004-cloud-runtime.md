# RFC-0004: Cloud Runtime — Design Document

Status: Draft
Author: Claudiu Groza (Vemeth)
Created: 2026-11-24
Target spec version: spec-0.1
Depends on: SPEC-00, SPEC-05 (Effects), SPEC-07 (Backends), SPEC-09 (Toolchain), SPEC-11 (Security)
Supersedes: none

## 1. Summary

This RFC proposes a Cloud Runtime for Lingua Omnibus, designed to enable deployment of LO programs to cloud infrastructure. The Cloud Runtime is **not** part of the language specification — it is an ecosystem component, similar to pons (RFC-0002).

The Cloud Runtime is deferred to post-v1.0 implementation. This document captures the design and API so that when the core compiler is stable, the cloud layer can be built without re-designing from scratch.

## 2. Motivation

### 2.1 Why not now

Cloud Runtime requires:
- A functional compiler that produces native binaries (✅ Phase 1 done)
- Multiple architecture backends (⚠️ Phase 2 in progress)
- A stable package manager (⚠️ lopm functional but basic)
- A security sandbox (⚠️ partial)

Implementing cloud orchestration before these are stable would be premature. The design must wait for the foundation.

### 2.2 Why design now

Even without implementation, defining the API and architecture now:
- Prevents ad-hoc decisions later
- Allows community feedback on the design
- Ensures compatibility with existing specs (effects, capabilities, security)
- Provides a roadmap for contributors

## 3. Architecture

### 3.1 Overview

```
┌─────────────────────────────────────────────────────┐
│                    Cloud Runtime                     │
├─────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐         │
│  │ Scheduler │  │ Load     │  │ Auto     │         │
│  │          │  │ Balancer │  │ Scaler   │         │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘         │
│       │              │              │               │
│  ┌────┴──────────────┴──────────────┴────┐         │
│  │           Node Manager                 │         │
│  │  (container lifecycle, health checks)  │         │
│  └────┬───────────────────────────────────┘         │
│       │                                             │
│  ┌────┴───────────────────────────────────┐         │
│  │           Network Layer                │         │
│  │  (service discovery, TLS, mTLS)        │         │
│  └────┬───────────────────────────────────┘         │
│       │                                             │
│  ┌────┴───────────────────────────────────┐         │
│  │           Storage Layer                │         │
│  │  (persistent volumes, object store)    │         │
│  └────────────────────────────────────────┘         │
└─────────────────────────────────────────────────────┘
```

### 3.2 Components

1. **Scheduler** — places workloads on nodes based on resource requirements
2. **Load Balancer** — distributes incoming requests across instances
3. **Auto-Scaler** — scales instances based on metrics (CPU, memory, request rate)
4. **Node Manager** — manages container lifecycle (start, stop, health check, restart)
5. **Network Layer** — service discovery, TLS termination, mTLS between nodes
6. **Storage Layer** — persistent volumes, object store, database connections

### 3.3 Container Model

Each LO program compiles to a container image:
- Base: minimal OS (Alpine or distroless)
- Binary: compiled LO program (x86_64, ARM64, etc.)
- Config: loc.toml with resource limits
- Health check: HTTP endpoint or exit code

## 4. Deployment API (proposed)

### 4.1 CLI commands

```bash
# Deploy to cloud
loc deploy --target=cloud --provider=aws --region=us-east-1

# Deploy to local cluster
loc deploy --target=local-cluster

# Scale
loc scale --instances=3

# Status
loc status

# Logs
loc logs --follow

# Rollback
loc rollback --version=1.2.0
```

### 4.2 loc.toml extensions

```toml
[cloud]
provider = "aws"           # aws, gcp, azure, self-hosted
region = "us-east-1"
instances = 2
cpu = "0.5"                # 0.5 cores
memory = "256Mi"           # 256 MB
port = 8080

[cloud.scaling]
min_instances = 1
max_instances = 10
target_cpu = 70            # scale up when CPU > 70%
target_rps = 1000          # scale up when RPS > 1000

[cloud.storage]
volumes = [
    { name = "data", size = "10Gi", mount = "/data" }
]

[cloud.networking]
tls = true
domains = ["api.example.com"]
```

### 4.3 Deployment manifest (cloud.yaml)

```yaml
apiVersion: lingua-omnibus.org/v1
kind: Deployment
metadata:
  name: my-app
  version: 1.0.0
spec:
  image: registry.example.com/my-app:1.0.0
  replicas: 2
  resources:
    cpu: "0.5"
    memory: "256Mi"
  ports:
    - containerPort: 8080
      protocol: TCP
  healthCheck:
    path: /health
    interval: 10s
    timeout: 5s
  env:
    - name: DATABASE_URL
      valueFrom: secret
```

## 5. Effects integration

The Cloud Runtime leverages LO's effects system (SPEC-05):

- `with [Network]` → requires network capability in container
- `with [Database]` → requires database connection
- `with [IO]` → requires file system access (configurable)
- `with [Quantum]` → requires quantum backend access

Effects declared in function signatures become container capabilities. A handler without `Network` cannot open outbound sockets — enforced by the security sandbox (SPEC-11).

## 6. Multilingual deployment

The Cloud Runtime inherits LO's multilingual-first design:

- Routes are multilingual (pons RFC-0002)
- Error messages are localized
- Health check endpoints are language-agnostic
- Logs are in the programmer's language (but structured for parsing)

## 7. Security

- Containers run as non-root
- Read-only file system by default (SEC-002)
- Capabilities granted via loc.toml [capabilities] (SEC-003)
- Network policies: deny-all by default, explicit allow rules
- TLS/mTLS between services
- Secret management: environment variables, mounted secrets
- No telemetry (Principle 7)

## 8. Observability

- Structured logging (JSON format, machine-parseable)
- Metrics endpoint (Prometheus-compatible)
- Distributed tracing (OpenTelemetry)
- Health checks (liveness, readiness)

## 9. Drawbacks and risks

1. **Scope creep**: Cloud runtime is a platform, not a framework. Must stay focused.
2. **Vendor lock-in**: Must support self-hosted deployment (Principle 1, 5).
3. **Complexity**: Orchestration is hard. Should leverage existing tools (Kubernetes, Docker) rather than reimplementing.
4. **Testing**: Requires cloud infrastructure for integration tests.

## 10. Rationale and alternatives

- **Alternative A: Kubernetes-native**: Deploy LO programs as Kubernetes pods. Rejected for MVP — too much complexity. Considered for v1.0.
- **Alternative B: Serverless (AWS Lambda, Cloud Functions)**: Good for simple functions, but LO needs long-running processes. Deferred.
- **Alternative C: Self-hosted only**: Simplest, but limits adoption. Considered for v0.1.
- **Alternative D: Docker containers**: Simple, portable, well-understood. **Selected for MVP.**

## 11. Implementation plan

| Phase | Milestone | Dependencies |
|-------|-----------|--------------|
| Phase 0 (now) | RFC + design doc | None |
| Phase 1 (post-v1.0) | Docker container build (`loc build --target=cloud`) | Backend x86_64, lopm stable |
| Phase 2 | Basic orchestration (start/stop/scale) | Phase 1, security sandbox |
| Phase 3 | Load balancing, auto-scaling | Phase 2, metrics |
| Phase 4 | Multi-region, persistent storage | Phase 3, LinguaDB clustering |

## 12. Impact on existing specifications

None. Cloud Runtime is an ecosystem component, not a language feature. No CONV, LEX, GRAM, TYPE, LIR, or other rules are added or modified.

## 13. Change log

| Version | Date | Summary |
|---------|------|---------|
| 0.1 | 2026-11-24 | Initial design document. Architecture, API, security, observability. |

End of RFC-0004 (Draft).