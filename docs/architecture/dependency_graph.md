# Dependency Graph System

SIS uses a directed dependency graph to determine service startup order and runtime relationships.

## Dependency Types

Services may depend on:

- other services
- hardware devices
- USB devices
- network availability
- Tor connectivity
- TPM attestation
- policy conditions
- time-based triggers

## Graph Properties

- Directed acyclic graph (DAG) preferred
- Cycle detection enforced at load time
- Dynamic updates allowed (with validation)

## Resolution Process

1. Parse all services
2. Build dependency graph
3. Detect cycles
4. Resolve execution order
5. Pass ordering to service manager

## Failure Handling

- Missing dependency → service blocked
- Circular dependency → system error or quarantine
- Policy violation → dependency denied

## Design Goal

Make service startup deterministic and explainable.
