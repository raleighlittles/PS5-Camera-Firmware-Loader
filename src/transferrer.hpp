#include <cstdint>
#include <libusb-1.0/libusb.h>

int ctrl_transfer_wrapper( libusb_device_handle* dev_handle,
                           uint8_t bmRequestType,
                           uint8_t bRequest,
                           uint16_t wValue,
                           uint16_t wIndex,
                           unsigned char* data,
                           uint16_t wLength)
{
    const int bytesWritten = libusb_control_transfer(dev_handle, bmRequestType, bRequest, wValue, wIndex, data, wLength, 0);

    if (bytesWritten == 0) {
        std::cerr << "ERROR: No bytes transferred" << std::endl;
    }

    else {
        // Libusb reported some kind of error (according to the documentation)
        // Variable name is a misnomer here
        if (bytesWritten < 0) {
            std::cerr << "ERROR: libusb reported error during control transfer, error# = " << bytesWritten << std::endl;
        }

        // libusb didn't report an error, but as a sanity check, make sure that the whole data chunk was transferred
        else if (bytesWritten != wLength) {
            // Libusb reported that the transfer went through correctly, but for some reason the entire firmware file wasn't transmitted.
            // Oddly, this 'error' can occur but the device appears to work otherwise just fine
            std::cout << "WARNING: libusb reported only " << bytesWritten << " bytes transferred, but firmware file is " << wLength << " bytes" << std::endl;
        }
    }

    return bytesWritten;
}