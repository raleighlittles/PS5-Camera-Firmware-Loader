
/** File: main.rs
 * 
 * 
 */

fn main() {

    let firmware_filename : String = std::env::args().nth(1).expect("No firmware filename received");

    let mut libusb_context : libusb::Context = libusb::Context::new().unwrap();

    libusb_context.set_log_level(libusb::LogLevel::Warning);

    // See `libusb` output in documentation for where these come from
    const USB_VENDOR_ID : u16 = 0x05a9;
    const USB_PRODUCT_ID : u16 = 0x0580;

    let mut libusb_dev_handle : libusb::DeviceHandle = libusb_context.open_device_with_vid_pid(USB_VENDOR_ID, USB_PRODUCT_ID).unwrap();

    // Device only has one USB 'endpoint'/interface (see `lsusb` output)
    const USB_INTERFACE_NUM : u8 = 0;

    /* Check if the device is in use by the kernel driver; if it is, then you'll
       need to "detach" it so that libusb can claim it.
       The device should almost never be claimed by the kernel driver unless you're already
       actively using it */

    if libusb_dev_handle.kernel_driver_active(USB_INTERFACE_NUM).unwrap() {

        println!("Kernel is currently using device, detaching it before continuing..");
        libusb_dev_handle.detach_kernel_driver(USB_INTERFACE_NUM).unwrap();

    }

    libusb_dev_handle.claim_interface(USB_INTERFACE_NUM).unwrap();

    // USB device is setup and ready to communicate to!

    let firmware_file_as_bytes : Vec<u8> = std::fs::read(firmware_filename).unwrap();

    // USB official standard limits packet size to 512 bytes
    const USB_MAX_PACKET_SIZE : u16 = 512;

    /* Constant comes from the bitmask: 0b1000000
       Result of setting 'Data Phase Transfer Detection' bit to 1, all others to 0
       See: https://www.beyondlogic.org/usbnutshell/usb6.shtml */
    const USB_OUTGOING_PACKET_BM_REQUEST_TYPE : u8 = 0x40;

    let firmware_file_len = firmware_file_as_bytes.len();

    /* To "load" the firmware onto the device, write out the first 64 KB of the firmware file
    to the device over USB, then send a footer packet */

    const TOTAL_BYTES_TO_WRITE : u16 = u16::MAX;

    if firmware_file_len < (TOTAL_BYTES_TO_WRITE as usize) {

        panic!("Firmware file size is insufficient. Expected={}, actual size={}", TOTAL_BYTES_TO_WRITE, firmware_file_len)
    }

    let mut file_byte_idx: usize = 0;

    // Not sure why this is the starting value. Constant was taken from OrbisEyeCam implementation
    let mut transaction_idx : u16 = 0x14; // 20d
    let mut wValue : u16 = 0;

    while file_byte_idx  < firmware_file_len {
        
        // let pkt_size = std::cmp::min(TOTAL_BYTES_TO_WRITE - file_byte_idx, MAX_USB_CHUNK_SIZE);

        let pkt_size : u16;

        if USB_MAX_PACKET_SIZE > (TOTAL_BYTES_TO_WRITE - file_byte_idx as u16) {
            pkt_size = TOTAL_BYTES_TO_WRITE - file_byte_idx as u16;
        } else {
            pkt_size = USB_MAX_PACKET_SIZE;
        }

        if (pkt_size == USB_MAX_PACKET_SIZE) {
            let bytes_transferred = libusb_dev_handle.write_control(USB_OUTGOING_PACKET_BM_REQUEST_TYPE, 0x0, wValue, transaction_idx, &firmware_file_as_bytes[file_byte_idx as usize .. (file_byte_idx.wrapping_add(pkt_size as usize))], std::time::Duration::ZERO).unwrap();
        } else {
            // Where adding would cause a wrap, simply add an extra byte to the transfer
            let mut special_chunk : Vec<u8>  = firmware_file_as_bytes[file_byte_idx as usize .. (file_byte_idx.wrapping_add(pkt_size as usize)) as usize].to_vec();
            special_chunk.push(firmware_file_as_bytes[(std::u16::MAX as usize + 1) as usize]);

            let bytes_transferred = libusb_dev_handle.write_control(USB_OUTGOING_PACKET_BM_REQUEST_TYPE, 0x0, wValue, transaction_idx, &special_chunk, std::time::Duration::ZERO).unwrap();
        }

        //transaction_idx += 1;
        wValue.wrapping_add(pkt_size as u16);
        file_byte_idx.wrapping_add(pkt_size as usize);

        println!("Transferred {} bytes, value= {}, index= {}", pkt_size, wValue, transaction_idx);
    }


    // for chunk in firmware_file_as_bytes[0..=std::u16::MAX as usize].chunks(USB_MAX_PACKET_SIZE as usize) {
    //     libusb_dev_handle.write_control(USB_OUTGOING_PACKET_BM_REQUEST_TYPE, 0x0, wValue, transaction_idx, chunk, std::time::Duration::ZERO).unwrap();
    //     wValue.checked_add(USB_MAX_PACKET_SIZE);
    // }

    // transaction_idx = 21;
    // wValue = 0;

    // for chunk in firmware_file_as_bytes[0 as usize..3584].chunks(USB_MAX_PACKET_SIZE as usize) {
    //     libusb_dev_handle.write_control(USB_OUTGOING_PACKET_BM_REQUEST_TYPE, 0x0, wValue, transaction_idx, chunk, std::time::Duration::ZERO).unwrap();
    //     wValue += USB_MAX_PACKET_SIZE;
    // }

    // Lastly, transmit footer packet
    let footer_packet : [u8 ; 1] = [0x5B];

    libusb_dev_handle.write_control(USB_OUTGOING_PACKET_BM_REQUEST_TYPE, 0x0, 0x2200, 0x8018, &footer_packet, std::time::Duration::ZERO).ok();


}