#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>

#include <libusb-1.0/libusb.h>

#include "src/transferrer.hpp"

constexpr static unsigned int CHUNK_SIZE = 512; // Bulk transfers are limited to 512 bytes per USB standard

int main(int argc, char* argv[]) {

    if (argc != 2) {
        std::cout << "ERROR: Incorrect number of arguments" << std::endl;
        return -1;
    }

    uint16_t productId = 0x0580;
    uint16_t vendorId = 0x05a9;

    libusb_context* lusb_context = nullptr;

    int rc = libusb_init(&lusb_context);

    libusb_set_option(lusb_context, LIBUSB_OPTION_LOG_LEVEL, LIBUSB_LOG_LEVEL_DEBUG);

    if (rc != 0) {
        std::cout << "ERROR: Failed to initialize libusb" << std::endl;
        return -1;
    }

    libusb_device_handle* lusb_dev_hndl = libusb_open_device_with_vid_pid(lusb_context, vendorId, productId);

    if (!lusb_dev_hndl) {
        std::cout << "ERROR: libusb device handler null"  << std::endl;
        return -1;
    }

    rc = libusb_claim_interface(lusb_dev_hndl, libusb_speed::LIBUSB_SPEED_UNKNOWN);

    if (rc != 0) {
        std::cout << "ERROR: Failed to claim device" << std::endl;
        return -1;
    }

    std::array<uint8_t, CHUNK_SIZE> chunk{};

    std::ifstream firmware_file(argv[1], std::ios::in | std::ios::binary | std::ios::ate);

    if (firmware_file.is_open())
    {
        uint32_t length = static_cast<uint32_t>(firmware_file.tellg());
        firmware_file.seekg(0, std::ios::beg);

        uint16_t index = 0x14;
        uint16_t value = 0;
        uint8_t usbWriteReqType = 0x40;

        for (uint32_t pos = 0; pos < length; pos += CHUNK_SIZE)
        {
            uint16_t size = (CHUNK_SIZE > (length - pos) ? (uint16_t)(length - pos) : CHUNK_SIZE);
            firmware_file.read(reinterpret_cast<char *>(chunk.data()), size);
            ctrl_transfer_wrapper(lusb_dev_hndl, usbWriteReqType, 0x0, value, index, chunk.data(), size);

            if ((static_cast<uint32_t>(value) + size) > 0xFFFF) index++;

            value += size;
        }

        firmware_file.close();

        chunk[0] = 0x5b;

        ctrl_transfer_wrapper(lusb_dev_hndl, usbWriteReqType, 0x0, 0x2200, 0x8018, chunk.data(), 1);

        std::cout << "Finished uploading firmware!" << std::endl;
    }
    else
    {
        std::cout << "Unable to open firmware" << std::endl;
        return -1;
    }

    return 0;
}
