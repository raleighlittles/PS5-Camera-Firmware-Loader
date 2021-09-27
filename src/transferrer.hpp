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
    int bytesWritten = libusb_control_transfer(dev_handle, bmRequestType, bRequest, wValue, wIndex, data, wLength, 1000);

    if (bytesWritten == 0) {
        std::cout << "ERROR: No bytes transferred" << std::endl;
    }

    else if (bytesWritten != wLength) {
        std::cout << "ERROR: Some bytes were not transmitted" << std::endl;
    }

    return bytesWritten;
}