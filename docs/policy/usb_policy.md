# USB Policy

USB policy defines how all USB devices are handled inside SIS.

---

## Default Behavior

USB is treated as hostile input by default.

- Unknown devices → denied or quarantined
- Only trusted or signed devices are allowed

---

## Policy Rules

```sis
usb {

    default: deny

    allow storage if device.signed == true

    allow hid if device.class == "keyboard"
        and user.approved == true

    require storage.mount if device.encrypted == true

    quarantine device if device.anomaly_detected == true

    audit usb.attach
    audit usb.detach
}
