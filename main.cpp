#include <iostream>
#include <libusb-1.0/libusb.h>
#include <cstdint>

int main() {

    uint16_t productId = 0x0580;
    uint16_t vendorId = 0x05a9;

    libusb_context* lusb_context = nullptr;

    int rc = libusb_init(&lusb_context);

    libusb_set_option(lusb_context, LIBUSB_OPTION_LOG_LEVEL, LIBUSB_LOG_LEVEL_DEBUG);

    if (rc != 0) {
        std::cout << "ERROR: Failed to init libusb" << std::endl;
        return -1;
    }

    libusb_device_handle* lusb_dev_hndl = libusb_open_device_with_vid_pid(lusb_context, vendorId, productId);

    if (!lusb_dev_hndl) {
        std::cout << "ERROR: libusb device handler null. "  << std::endl;
        return -1;
    }

    // Claim device
    rc = libusb_claim_interface(lusb_dev_hndl, 0);

    if (rc != 0) {
        std::cout << "ERROR: Failed to claim device" << std::endl;
        return -1;
    }

    return 0;
}
