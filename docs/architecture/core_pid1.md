# Core PID 1 Design

PID 1 in SIS is intentionally minimal and acts only as the trusted system orchestrator.

## Responsibilities

PID 1 is responsible for:

- Boot orchestration
- Service lifecycle supervision
- Dependency resolution
- Module loading
- Event dispatching
- Policy enforcement coordination
- System shutdown and recovery

## What PID 1 does NOT do

- No business logic
- No application-level functionality
- No direct device handling
- No network stack implementation

## Runtime Loop Concept

PID 1 operates as a deterministic event loop:

1. Initialize kernel handoff
2. Load core configuration
3. Start event bus
4. Load policy engine
5. Resolve service dependency graph
6. Start services in order
7. Enter supervision loop

## Design Principle

> PID 1 should be small enough to audit manually.
