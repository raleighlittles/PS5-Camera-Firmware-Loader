extern crate libusb;

fn main() {

    let firmware_filename = std::env::args().nth(1).expect("No firmware filename received");

    let mut libusb_context : libusb::Context = libusb::Context::new().unwrap();

    libusb_context.set_log_level(libusb::LogLevel::Debug);

    const usbVendorId : u16 = 0x05a9;
    const usbProductId : u16 = 0x0580;

    let libusb_dev_handle : Option<libusb::DeviceHandle> = libusb_context.open_device_with_vid_pid(usbVendorId, usbProductId);

    const usbInterfaceNum : u8 = 0;

}