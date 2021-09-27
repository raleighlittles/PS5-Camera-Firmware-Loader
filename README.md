# About

This tool is used to load firmware onto a PS5 camera.

![ps5-camera](https://gmedia.playstation.com/is/image/SIEPDC/hd-camera-ps5-image-block-03-en-02jul20?$facebook$)

The main reason why you'd want to load custom firmware onto the camera is to be able to use it as a UVC device.

This is a Linux port of [OrbisEyeCam](https://github.com/psxdev/OrbisEyeCam) for Windows. Kudus to @psxdev for the initial effort of reverse-engineering it.

# Usage as a webcam

To use this as a webcam, we'll load a custom firmware onto the device. Download `firmware.bin` from here: https://github.com/Hackinside/PS5_camera_files/blob/main/firmware.bin

(Kudos to @Hackinside for the custom firmware)

## Connect device

Connect your PS5 camera to your PC via USB.

Make sure you see the following in the dmesg log:

```
 usb 2-4.4.4.4: new SuperSpeed Gen 1 USB device number 7 using xhci_hcd
 usb 2-4.4.4.4: New USB device found, idVendor=05a9, idProduct=0580, bcdDevice= 1.00
 usb 2-4.4.4.4: New USB device strings: Mfr=1, Product=2, SerialNumber=0
 usb 2-4.4.4.4: Product: USB Boot
 usb 2-4.4.4.4: Manufacturer: OmniVision Technologies, Inc.
```
Keep the dmesg window open, we'll need it for later.
## Setup permissions

[libusb](https://libusb.info/) needs permissions to be able to write to USB devices.

0. Make sure your user is part of the `plugdev` group.

1. Copy the udev rules (`100-playstation-camera.rules`) to `/etc/udev/rules.d`

2. Reload the udev rules by running: 

`$ sudo udevadm control --reload ; sudo udevadm trigger `

## Run the script

Build and run this project using CMake.

```bash
$ cmake CMakeLists.txt
$ make 
$ ./ps5_camera_firmware_loader <firmware-file-path>
```

## Success 	:heavy_check_mark:

Go back to the dmesg window from earlier. You should see the following line:

```
uvcvideo: Found UVC 1.00 device USB Camera-OV580 (05a9:058c)
```

Open your favorite webcam program and now you're all set.