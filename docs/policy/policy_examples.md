# SIS Policy Examples

This file shows practical examples of SIS-Policy usage.

---

## Default High-Security Policy

```sis
policy "high-security" {

    default {
        service: deny
        device: deny
        network: deny
    }

    rules {

        allow service if service.signed == true
        allow service if service.policy_compliant == true

        require service if service.dependencies_met == true

        audit service.start
        audit policy.violation

        quarantine service if service.crash_count > 3
    }
}
