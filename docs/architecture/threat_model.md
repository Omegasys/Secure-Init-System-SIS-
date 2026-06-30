# Threat Model

SIS is designed with explicit assumptions about adversaries and system compromise.

## Threat Assumptions

We assume attackers may:

- Inject malicious USB devices
- Compromise user-space services
- Attempt network exfiltration
- Modify untrusted modules
- Exploit misconfigured services

## Mitigation Strategies

### USB threats
- Device classification system
- Default deny policy
- Quarantine unknown devices

### Network threats
- Tor enforcement for anonymous profiles
- DNS leak detection
- Firewall policy enforcement

### Module threats
- Signature verification
- Policy gating
- Runtime isolation

### Service threats
- Crash monitoring
- Auto-quarantine
- Capability restrictions

## Trust Boundary

The only trusted component is:

> PID 1 + SIS core + policy engine

Everything else is untrusted by default.

## Design Goal

Assume compromise outside the core and enforce recovery and containment automatically.
