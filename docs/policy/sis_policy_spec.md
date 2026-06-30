# SIS Policy Specification (SIS-Policy)

SIS-Policy is the central declarative security and behavior language for the SIS init system.

It defines what the system is allowed to do, under what conditions, and how violations are handled.

---

## Core Philosophy

SIS-Policy is:

- Declarative (not procedural)
- Deterministic
- Event-driven
- Capability-based
- Fully auditable

There is no hidden logic outside this specification.

---

## Policy Actions

SIS-Policy supports five core actions:

- allow → permit an operation
- deny → block an operation
- require → enforce a condition before execution
- audit → log the event
- quarantine → isolate the subject

---

## Objects

Policies operate on:

- service
- module
- device
- usb
- network
- process
- user
- event

---

## Conditions

Conditions are boolean expressions:

- ==
- !=
- <
- >
- <=
- >=
- has
- in
- exists
- matches
- signed
- trusted
- attested

---

## Execution Model

1. Event occurs
2. Policy engine evaluates rules
3. Decision is computed
4. Enforcement is applied immediately

---

## Design Goal

Replace fragmented security systems (SELinux, firewall rules, udev rules) with one unified control language.
