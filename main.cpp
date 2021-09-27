#include <iostream>
#include <fstream>
#include <cstdint>

#include <libusb-1.0/libusb.h>
#include <array>

#include "transferrer.h"

constexpr static unsigned int CHUNK_SIZE = 512; // Bulk transfers are limited to 512 bytes per USB standard

int main() {

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

    // Claim device
    rc = libusb_claim_interface(lusb_dev_hndl, 0);

    if (rc != 0) {
        std::cout << "ERROR: Failed to claim device" << std::endl;
        return -1;
    }

    //uint32_t CHUNK_SIZE = 512; // Bulk transfers are limited to 512 bytes per USB standard
    // uint8_t chunk[CHUNK_SIZE];
    std::array<uint8_t, CHUNK_SIZE> chunk{};

    std::string firmware_filename = "/home/raleigh/CLionProjects/ps5-camera/firmware.bin";
    std::ifstream firmware_file(firmware_filename.c_str(), std::ios::in | std::ios::binary | std::ios::ate);

    if (firmware_file.is_open())
    {
        uint32_t length = (uint32_t)firmware_file.tellg();
        firmware_file.seekg(0, std::ios::beg);

        uint16_t index = 0x14;
        uint16_t value = 0;

        for (uint32_t pos = 0; pos < length; pos += CHUNK_SIZE)
        {
            uint16_t size = (CHUNK_SIZE > (length - pos) ? (uint16_t)(length - pos) : CHUNK_SIZE);
            //firmware_file.read((char*)chunk, size);
            firmware_file.read(reinterpret_cast<char *>(chunk.data()), size);
            // submitAndWait_controlTransfer(0x40, 0x0, value, index, size, chunk);
            //ctrl_transfer_wrapper(lusb_dev_hndl, 0x40, 0x0, value, index, chunk, size);
            ctrl_transfer_wrapper(lusb_dev_hndl, 0x40, 0x0, value, index, chunk.data(), size);
            //if (((uint32_t)value + size) > 0xFFFF) index += 1;
            if ((static_cast<uint32_t>(value) + size) > 0xFFFF) index++;

            value += size;
        }
        firmware_file.close();

        chunk[0] = 0x5b;
        // submitAndWait_controlTransfer(0x40, 0x0, 0x2200, 0x8018, 1, chunk);
        //ctrl_transfer_wrapper(lusb_dev_hndl, 0x40, 0x0, 0x2200, 0x8018, chunk, 1);
        ctrl_transfer_wrapper(lusb_dev_hndl, 0x40, 0x0, 0x2200, 0x8018, chunk.data(), 1);

        std::cout << "Firmware uploaded..." << std::endl;
    }
    else
    {
        std::cout << "Unable to open firmware.bin!" << std::endl;
        return -1;
    }

    return 0;
}
