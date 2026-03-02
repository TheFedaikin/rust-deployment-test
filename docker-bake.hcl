variable "TAG" {
  default = "latest"
}

variable "REGISTRY" {
  default = "ghcr.io/thefedaikin"
}

# Strategy A: full multi-stage build inside Docker
target "server" {
  dockerfile = "Dockerfile"
  tags       = ["${REGISTRY}/axum-deployment:${TAG}", "${REGISTRY}/axum-deployment:latest"]
  output     = ["type=registry"]
}

target "server-local" {
  inherits = ["server"]
  tags     = ["axum-deployment:local"]
  output   = ["type=docker"]
}

# Strategy B: pre-built binary, thin runtime image
target "server-prebuilt" {
  dockerfile = "Dockerfile.runtime"
  tags       = ["${REGISTRY}/axum-deployment-prebuilt:${TAG}", "${REGISTRY}/axum-deployment-prebuilt:latest"]
  output     = ["type=registry"]
}

target "server-prebuilt-local" {
  inherits = ["server-prebuilt"]
  tags     = ["axum-deployment-prebuilt:local"]
  output   = ["type=docker"]
}
