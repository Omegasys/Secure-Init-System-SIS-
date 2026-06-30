# Event Bus

SIS uses a global event bus as the backbone of system communication.

## Event Model

Everything in SIS is event-driven:

- service lifecycle events
- hardware events
- network state changes
- policy violations

## Example Events

- usb.attach
- usb.detach
- net.up
- net.down
- service.start
- service.fail
- policy.violation
- boot.stage.completed

## Event Behavior

Events can:

- Trigger services
- Block execution
- Modify system state
- Trigger policy evaluation

## Delivery Semantics

- Ordered delivery per event channel
- Optional async dispatch
- Policy hooks executed before execution

## Design Goal

Replace polling-based system design with deterministic event propagation.
