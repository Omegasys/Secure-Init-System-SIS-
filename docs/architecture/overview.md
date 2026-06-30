# SIS Architecture Overview

SIS (Secure Init System) is a modular, policy-driven init system that replaces traditional monolithic init systems with a minimal PID 1 core and an event-driven control plane.

## Core Idea

SIS separates the system into three layers:

1. **Core PID 1**
   - Minimal trusted runtime
   - Boot orchestration
   - Service supervision
   - Policy enforcement hook

2. **Modules**
   - Optional system capabilities (networking, USB, Tor, TPM, containers)
   - Fully replaceable and versioned
   - Loaded dynamically or at boot

3. **Policy Engine (SIS-Policy)**
   - Central decision layer
   - Controls all system behavior
   - Declarative allow/deny/require rules

## Key Properties

- Deterministic boot process
- Fully auditable system state
- Capability-based security model
- Event-driven architecture
- No hidden system behavior

## Design Goal

Replace fragmented Linux subsystems (systemd, udev, SELinux, firewall rules) with a single unified control plane.
