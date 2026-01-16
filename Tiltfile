# Tiltfile for asc-mcp local development
# Usage: tilt up

# Build configuration
docker_build(
    'asc-mcp',
    '.',
    dockerfile='Dockerfile',
    live_update=[
        sync('.', '/app'),
        run('cd /app && go build -o /asc-mcp ./cmd/asc-mcp'),
    ],
)

# Load Kubernetes manifests from ops/k8s
k8s_yaml([
    'ops/k8s/namespace.yaml',
    'ops/k8s/secret.yaml',
    'ops/k8s/configmap.yaml',
    'ops/k8s/deployment.yaml',
    'ops/k8s/service.yaml',
])

# Configure the asc-mcp resource
k8s_resource(
    'asc-mcp',
    port_forwards=[],
    labels=['asc-mcp'],
)

# Local resource for running tests
local_resource(
    'test',
    cmd='./script/test.zsh',
    deps=['internal/', 'cmd/'],
    labels=['test'],
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
)

# Local resource for linting
local_resource(
    'lint',
    cmd='go vet ./... && go fmt ./...',
    deps=['internal/', 'cmd/'],
    labels=['lint'],
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
)

# Local resource for building locally (outside container)
local_resource(
    'build-local',
    cmd='./script/build.zsh',
    deps=['internal/', 'cmd/', 'go.mod', 'go.sum'],
    labels=['build'],
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
)
