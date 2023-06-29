fn main() {
    let firmware_filename: String = std::env::args()
        .nth(1)
        .expect("No firmware filename received");

    let mut libusb_context: libusb::Context = libusb::Context::new().unwrap();

    libusb_context.set_log_level(libusb::LogLevel::Warning);

    // See `libusb` output in documentation for where these come from
    const USB_VENDOR_ID: u16 = 0x05a9;
    const USB_PRODUCT_ID: u16 = 0x0580;

    let mut libusb_dev_handle: libusb::DeviceHandle = libusb_context
        .open_device_with_vid_pid(USB_VENDOR_ID, USB_PRODUCT_ID)
        .unwrap();

    // Device only has one USB 'endpoint'/interface (see `lsusb` output)
    const USB_INTERFACE_NUM: u8 = 0;

    /* Check if the device is in use by the kernel driver; if it is, then you'll
    need to "detach" it so that libusb can claim it.
    The device should almost never be claimed by the kernel driver unless you're already
    actively using it */

    if libusb_dev_handle
        .kernel_driver_active(USB_INTERFACE_NUM)
        .unwrap()
    {
        println!("Kernel is currently using device, detaching it before continuing..");
        libusb_dev_handle
            .detach_kernel_driver(USB_INTERFACE_NUM)
            .unwrap();
    }

    libusb_dev_handle
        .claim_interface(USB_INTERFACE_NUM)
        .unwrap();

    // USB device is setup and ready to communicate to!

    let firmware_file_as_bytes: Vec<u8> = std::fs::read(firmware_filename).unwrap();

    // USB official standard limits packet size to 512 bytes
    const USB_MAX_PACKET_SIZE: u16 = 512;

    /* Constant comes from the bitmask: 0b1000000
    Result of setting 'Data Phase Transfer Detection' bit to 1, all others to 0
    See: https://www.beyondlogic.org/usbnutshell/usb6.shtml */
    const USB_OUTGOING_PACKET_BM_REQUEST_TYPE: u8 = 0x40;

    let firmware_file_len = firmware_file_as_bytes.len();

    // Firmware files need to be at least 64 KB in size
    const MIN_FIRMWARE_FILE_SIZE: u16 = u16::MAX;

    if firmware_file_len < (MIN_FIRMWARE_FILE_SIZE as usize) {
        panic!(
            "Firmware file size is insufficient. Expected={}, actual size={}",
            MIN_FIRMWARE_FILE_SIZE, firmware_file_len
        )
    }

    let mut file_byte_idx: usize = 0;

    // Not sure why this is the starting value. Constant was taken from OrbisEyeCam implementation
    let lower_transaction_idx: u16 = 0x14; // 20d
    let upper_transaction_idx = 0x15; // 21d

    /* Goes from 0 to 65536, incrementing by 512. Then, starts over at 0, and continues incrementing.
    This is again something that was taken from OrbisEyeCam */
    let mut wValue = std::num::Wrapping(std::u16::MAX);
    wValue.0 = 0;

    while file_byte_idx < firmware_file_len {
        let pkt_size = std::cmp::min(
            firmware_file_len - file_byte_idx,
            USB_MAX_PACKET_SIZE as usize,
        );

        let pkt_end_idx: usize = file_byte_idx + (pkt_size as usize);

        let cur_transaction_idx = if file_byte_idx < (std::u16::MAX as usize) {
            lower_transaction_idx
        } else {
            upper_transaction_idx
        };

        let bytes_transferred = libusb_dev_handle
            .write_control(
                USB_OUTGOING_PACKET_BM_REQUEST_TYPE,
                0x0,
                wValue.0,
                cur_transaction_idx,
                &firmware_file_as_bytes[file_byte_idx as usize..pkt_end_idx],
                std::time::Duration::ZERO,
            )
            .unwrap();

        // Careful with this log statement. Logging in between USB transactions can slow things down enough to where it no longer works
        //println!("Transferred {} bytes [{} , {}], value= {}, index= {}", bytes_transferred, file_byte_idx, pkt_end_idx, wValue.0, transaction_idx);

        wValue += pkt_size as u16;
        file_byte_idx += pkt_size as usize;
    }

    // Again, taken from OrbisEyeCam
    let footer_packet: [u8; 1] = [0x5B];

    libusb_dev_handle
        .write_control(
            USB_OUTGOING_PACKET_BM_REQUEST_TYPE,
            0x0,
            0x2200,
            0x8018,
            &footer_packet,
            std::time::Duration::ZERO,
        )
        .unwrap();
}
