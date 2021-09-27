//
// Created by raleigh on 9/26/21.
//

#include <cstdint>
#include <libusb-1.0/libusb.h>

#ifndef PS5_CAMERA_TRANSFERRER_H
#define PS5_CAMERA_TRANSFERRER_H

int ctrl_transfer_wrapper( libusb_device_handle* dev_handle,
                                  uint8_t bmRequestType,
                                  uint8_t bRequest,
                                  uint16_t wValue,
                                  uint16_t wIndex,
                                  unsigned char* data,
                                  uint16_t wLength);

#endif //PS5_CAMERA_TRANSFERRER_H
