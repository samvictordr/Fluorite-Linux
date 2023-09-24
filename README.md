# Welcome to The Mesa Playground!
This repo consists mostly of random OSdev projects (An OSdev playground, if you will)

## mesakernel 
A minimal kernel made using rust, with the help of [blog_os](https://os.phil-opp.com/) - 
If you want to run the bootable image, download the .bin file at 'mesakernel/x86_64-generic/bootimage-mesakernel.bin'. Make sure you have QEMU installed or a USB flash drive to make a bootable USB.

Running using QEMU:
Run this command in a terminal
```
 qemu-system-x86_64 -drive format=raw,file=/path/to/bootimage-mesakernel.bin
```
Flashing to a USB drive using dd:
```
dd if=/path/to/bootimage-mesakernel.bin of=/dev/sdx && sync
```
Where sdx is the drive name of your USB. Double check your drive before flashing it, dd is infamous for doing undesirable things.
