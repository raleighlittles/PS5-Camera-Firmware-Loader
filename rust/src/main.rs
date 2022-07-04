

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

    const max_usb_chunk_size : u16 = 512;

    // Note: Rust doesn't let you modify the value of the index inside of a for loop

    let mut idx = 0;
    let mut length = firmware_file_as_bytes.len();

    while (idx < length) {

        // Transmit up to as many bytes as you can
        // Note: Really wish Rust had a ternary operator
    }

    

}