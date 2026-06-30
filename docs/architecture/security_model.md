# Security Model

SIS uses a layered security model combining kernel enforcement, module isolation, and policy-driven control.

## Security Layers

### 1. Kernel Layer
- Enforces process isolation
- Provides basic system security primitives

### 2. SIS Module Layer
- Sandboxes all modules
- Controls capabilities per module
- Prevents direct system access

### 3. SIS-Policy Layer
- Central decision engine
- Evaluates all system actions
- Enforces allow/deny/require rules

## Capability System

Permissions are assigned per:

- service
- module
- device
- network
- user session

## Trust Levels

- untrusted
- restricted
- trusted
- verified
- attested (TPM-backed)

## Design Goal

Make system behavior explicitly declared rather than implicitly allowed.
