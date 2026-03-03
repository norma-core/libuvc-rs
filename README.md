# norm-uvc

[![crates.io](https://img.shields.io/crates/v/norm-uvc.svg)](https://crates.io/crates/norm-uvc)
[![license](https://img.shields.io/crates/l/norm-uvc.svg)](https://github.com/norma-core/libuvc-rs/blob/master/LICENSE)

Safe Rust wrapper around the [norma-core libuvc fork](https://github.com/norma-core/libuvc) with extended timestamp support.

## ⚠️ Linux Only

This library **only supports Linux** as it uses `CLOCK_BOOTTIME` timestamps from the norma-core libuvc fork.

## What does this library do?

This library provides access to webcams and allows capturing video streams with precise timestamps.

### Permissions

An `Access` error might be due to insufficient USB device permissions. Grant access with:

```bash
chmod 0666 /dev/bus/usb/{BUS}/{DEVICE}
```

Find BUS and DEVICE with `lsusb`.

## Features

- Always vendors libuvc source (no system dependency)
- Extended timestamp support via `CLOCK_BOOTTIME`
- `uvc_debugging` feature for detailed logging

## Documentation

Generate documentation with `cargo doc`

## Credits

Forked from [libuvc-rs](https://github.com/mulimoen/libuvc-rs) by Magnus Ulimoen.
Extended with timestamp support by NormaCore.Dev.
