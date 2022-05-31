# TODO

- set the boot order of the NVRAM. Can technically use VARS.fd but efivars is fine
- set the stack pointer up at 0x6M. Then go into EL2

## C & ASM Stubs

Its possible to precompile C sources. Then link those object together as well as a header API. ASM can be included into rust directly. And global asm is apparently stable now. So I guess Im just doing it wrong.

### Combining UEFI

uefi-rs seems to implement a lot of the specs already.
[Here](https://retrage.github.io/edk2-nightly/) is a convenient place to get the nightly EDK2 images.

I think its basically `-disk if=bios.bin` for OVMF_EFI.fd.

On Mac, you just have to do:

```bash
mkdir -p diskImage/EFI/BOOT
cp bootx64.efi diskImage/EFI/BOOT/BOOTX64.EFI
hdiutil create -fs fat32 -ov -size 48m -volname NEWOS -format UDTO -srcfolder diskImage uefi.cdr
```

## Export API

Arcboot exports an API with 3 modules and a few functions to help setup an arcboot compliant kernel and to then request services from the arcboot environment. The arcboot env includes the SEE on riscv and seems like limine.

REMOVE:

```rust
fn check_screenshot(image: Handle, bt: &BootServices, name: &str) {
    let serial_handles = bt
        .find_handles::<Serial>()
        .expect("Failed to get serial handles");

    // Use the second serial device handle. Opening a serial device
    // in exclusive mode breaks the connection between stdout and
    // the serial device, and we don't want that to happen to the
    // first serial device since it's used for log transport.
    let serial_handle = *serial_handles
        .get(1)
        .expect("Second serial device is missing");

    let mut serial = bt
        .open_protocol::<Serial>(
            OpenProtocolParams {
                handle: serial_handle,
                agent: image,
                controller: None,
            },
            OpenProtocolAttributes::Exclusive,
        )
        .expect("Could not open serial protocol");

    // Set a large timeout to avoid problems with Travis
    let mut io_mode = *serial.io_mode();
    io_mode.timeout = 10_000_000;
    serial
        .set_attributes(&io_mode)
        .expect("Failed to configure serial port timeout");

    // Send a screenshot request to the host
    serial
        .write(b"SCREENSHOT: ")
        .expect("Failed to send request");
    let name_bytes = name.as_bytes();
    serial.write(name_bytes).expect("Failed to send request");
    serial.write(b"\n").expect("Failed to send request");

    // Wait for the host's acknowledgement before moving forward
    let mut reply = [0; 3];
    serial
        .read(&mut reply[..])
        .expect("Failed to read host reply");

    assert_eq!(&reply[..], b"OK\n", "Unexpected screenshot request reply");

    bt.stall(3_000_000);
}
```
