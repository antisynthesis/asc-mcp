# ADR-0006: Kubernetes Deployment Model

## Status

Accepted

## Context

While MCP servers typically run as local subprocesses, there are scenarios where running as a service is beneficial:

- Shared access across multiple clients
- Centralized credential management
- Cloud-native deployment
- Integration with CI/CD pipelines

The stdio transport model complicates direct Kubernetes deployment since there's no network listener.

## Decision

We will provide Kubernetes manifests for deployment with the following considerations:

1. **Deployment Model**: Single replica deployment
   - The server runs but doesn't expose network ports for MCP
   - Useful for batch operations, health checks, and future HTTP transport

2. **Credential Management**:
   - Kubernetes Secret for ASC credentials
   - Private key stored as secret data
   - ConfigMap for non-sensitive configuration

3. **Manifests Provided**:
   - `namespace.yaml` - Dedicated namespace
   - `secret.yaml` - Template for credentials (user must populate)
   - `configmap.yaml` - Configuration values
   - `deployment.yaml` - Application deployment
   - `service.yaml` - Service definition (headless, no ports)

4. **Local Development**:
   - Tiltfile for Tilt-based development workflow
   - Automatic rebuild on code changes
   - Port-forward support if HTTP added later

## Consequences

### Positive

- Production-ready deployment templates
- Proper secret management
- Namespace isolation
- Foundation for future HTTP transport
- Tilt enables rapid local iteration

### Negative

- Current stdio transport limits direct K8s usage
- Overhead of K8s for single-instance deployment
- Secrets require manual population

### Future Considerations

- Add HTTP+SSE transport for network access
- Add Ingress for external exposure
- Add HorizontalPodAutoscaler for scaling
- Add NetworkPolicy for security
