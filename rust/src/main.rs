#![deny(warnings)]

fn main() {

    let firmware_filename : String = std::env::args().nth(1).expect("No firmware filename received");

    let mut libusb_context : libusb::Context = libusb::Context::new().unwrap();

    libusb_context.set_log_level(libusb::LogLevel::Debug);

    const usbVendorId : u16 = 0x05a9;
    const usbProductId : u16 = 0x0580;

    let mut libusb_dev_handle : libusb::DeviceHandle = libusb_context.open_device_with_vid_pid(usbVendorId, usbProductId).unwrap();

    // Device only has one USB 'endpoint'/interface (see `lsusb` output)
    const usbInterfaceNum : u8 = 0;

    /* Check if the device is in use by the kernel driver; if it is, then you'll
       need to "detach" it so that libusb can claim it.
       The device should almost never be claimed by the kernel driver unless you're already
       actively using it */

    if (libusb_dev_handle.kernel_driver_active(usbInterfaceNum).unwrap()) {

        println!("Kernel is currently using device, detaching it before continuing..");
        libusb_dev_handle.detach_kernel_driver(usbInterfaceNum);
    }

    libusb_dev_handle.claim_interface(usbInterfaceNum).unwrap();

    // USB device is setup and ready to communicate to!

    let firmware_file_as_bytes : Vec<u8> = std::fs::read(firmware_filename).unwrap();

    const max_usb_chunk_size : usize = 512;

    /* Constant comes from the bitmask: 0b1000000
       Result of setting 'Data Phase Transfer Detection' bit to 1, all others to 0
       See: https://www.beyondlogic.org/usbnutshell/usb6.shtml */
    const usb_outgoing_packet_bmRequestType : u8 = 0x40;

    // Note: Rust doesn't let you modify the value of the index inside of a for loop

    let mut idx : u16 = 0x14; // 20d
    let mut value : u16 = 0;
    let mut length : u32 = usize::try_into(firmware_file_as_bytes.len()).unwrap();

    while ((idx as u32) < length) {

        // Transmit up to as many bytes as you can
        let usbPacketSize : u16;

        let bytesRemaining = length - (idx as u32);

        if (max_usb_chunk_size > bytesRemaining as usize) {
            usbPacketSize = bytesRemaining as u16;
        }
        else {
            usbPacketSize = max_usb_chunk_size as u16;
        }

        // Magic numbers; not entirely sure where they come from -- likely device-specific. Taken from original Windows implementation
        let bytesTransferred = libusb_dev_handle.write_control(usb_outgoing_packet_bmRequestType, 0x0, value, idx, &firmware_file_as_bytes[idx as usize .. (idx + usbPacketSize) as usize], std::time::Duration::ZERO).unwrap();

        if (bytesTransferred < 1) {
            panic!("libusb encountered an error during transmission, some bytes were not correctly sent");
        }

        idx += max_usb_chunk_size as u16;
        value += usbPacketSize
    }

    // Lastly, transmit header byte
    let footer_packet : [u8 ; 1] = [0x5B];
    if (libusb_dev_handle.write_control(usb_outgoing_packet_bmRequestType, 0x0, 0x2200, 0x8018, &footer_packet, std::time::Duration::ZERO).unwrap() != footer_packet.len()) {
        panic!("Failed to transmit footer byte")
    }


}