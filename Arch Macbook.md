# Macbook Arch Guide - by gilbertw1

- [Installation](#installation)
- [General Setup](#general-setup)
- [System Time](#system-time)
- [Display](#display)
- [Keyboard](#keyboard)
- [Touchpad](#touchpad)
- [Sound](#sound)
- [Bluetooth](#bluetooth)
- [Facetime HD](#facetime-hd)
- [Networking](#networking)
- [Power](#power)
- [Printing](#printing)
- [Random](#random)
- [Appearance](#appearance)
- [HiDPI](#hidpi)
- [Xmonad Startup](#xmonad-startup)
- [Packages](#packages)
- [Keybindings](#keybindings)


## Installation

    cgdisk /dev/sda

    # Setup Partitions
    # 1 100MiB EFI partition # Hex code ef00
    # 2 250MiB Boot partition # Hex code 8300
    # 3 4GiB Swap partition # Hex code 8200
    # 3 100% size partiton # Hex code 8300

    # format paritions
    mkfs.vfat -F32 /dev/sda1
    mkfs.ext2 /dev/sda2
    mkfs.ext4 /dev/sda4

    # setup swap
    mkswap /dev/sda3
    swapon /dev/sda3

    # mount paritions
    mount /dev/sda4 /mnt
    mkdir /mnt/boot
    mount /dev/sda2 /mnt/boot
    mkdir /mnt/boot/efi
    mount /dev/sda1 /mnt/boot/efi

    # install arch linux
    pacstrap /mnt base base-devel grub-efi-x86_64 zsh zsh-completions vim git efibootmgr dialog wpa_supplicant

    # fstab
    genfstab -pU /mnt >> /mnt/etc/fstab

    # enter the new system
    arch-chroot /mnt /bin/bash

    # set hostname
    echo MYHOSTNAME > /etc/hostname

    # set locale
    echo LANG=en_US.UTF-8 >> /etc/locale.conf
    echo LANGUAGE=en_US >> /etc/locale.conf
    echo LC_ALL=C >> /etc/locale.conf

    vim /etc/locale.gen
    # uncomment en_US.UTF-8
    locale-gen

    # set root password
    passwd

    # create user
    useradd -m -g users -G wheel,storage,power -s /bin/zsh MYUSERNAME
    passwd MYUSERNAME

    # Regenerate initrd image
    mkinitcpio -p linux

    # setup grub
    grub-install
    grub-mkconfig -o /boot/grub/grub.cfg

    # unmount paritions and restart (remove usb)
    exit
    umount -R /mnt
    reboot



## General Setup

**XOrg**

    pacman -S xorg-server xorg-xinit xorg-server-utils mesa
    pacman -S xorg-twm xorg-xclock xterm # basic components

**XMonad**

    pacman -S xmonad xmonad-contrib xmobar trayer

**Awesome WM**

    sudo pacman -S awesome vicious

**XFCE 4 (Backup + Utilities)**

    sudo pacman -S xfce4 xfce4-goodies

**Start WM with X**

    echo "exec xmonad" >> ~/.xinitrc # OR
    echo "exec awesome" >> ~/.xinitrc # OR
    echo "exec startxfce4" >> ~/.xinitrc

**Create Awesome Config**

    mkdir -p ~/.config/awesome/
    cp /etc/xdg/awesome/rc.lua ~/.config/awesome/ # basic config

**Community Packages**

    sudo pacman -S yaourt

**Enable 64-bit wrapped 32-bit packages + community repo (uncomment / add)**

    vim /etc/pacman.conf

    # [multilib]
    # Include = /etc/pacman.d/mirrorlists

    # [archlinuxfr]
    # SigLevel = Never
    # Server = http://repo.archlinux.fr/$arch

**Suspend**

    systemctl suspend



## System Time

**Set Time**

    tzselect  $ US -> Eastern
    sudo ln -s /usr/share/zoneinfo/US/Eastern /etc/localtime
    hwclock --systohc --utc



## Display

**Brightness**

    yaourt -S light-git
    light -S 50  # sets brightness to 50%
    light -U 10 # decrease by 10%
    light -A 10 # increase by 10%


**XScreensaver**

    sudo pacman -S xscreensaver
    xscreensaver-demo

**Lock Screen (On suspend/hibernate)**

    yaourt -S xss-lock
    xss-lock -- /usr/bin/xscreensaver-command -lock &

**Lock**

    xscreensaver-command --lock

**Disable LID Wakeup (Prevents proper suspend when lid open)**

    sudo vim etc/tmpfiles.d/disable-lid0-wake.conf

    # w /proc/acpi/wakeup - - - - LID0


## Keyboard

**Backlight (startup)**

    kbdlight max

**Fix Tilde Key & Fix F# Keys**

    sudo vim /etc/modprobe.d/hid_apple.conf

    # options hid_apple iso_layout=0
    # options hid_apple fnmode=2



## Touchpad

**Basic Support**

    pacman -S xf86-input-synaptics

**Natural Scrolling**

    xinput set-button-map 12 1 2 3 5 4 7 6



## Sound

**PulseAudio**

    sudo pacman -S pulseaudio pulseaudio-bluetooth pulseaudio-alsa paprefs
    yaourt -S pasystray

**Control**

    sudo pacman -S alsa-utils
    alsamixer

**Volume Applet**

    sudo pacman -S pasystray



## Bluetooth

**Install Bluetooth Packages**

    sudo pacman -S bluez bluez-utils

**Start and Enable Bluetooth Service**

    systemctl enable bluetooth
    systemctl start bluetooth

**Install and Run Blueman**

    sudo pacman -S blueman blueman-applet

**Start Blueman Applet (startup)**

    blueman-applet



## Facetime HD

**Install Experimental Facetime HD Driver**

    yaourt -S bcwc-pcie-git



## Networking

**Broadcom Drivers**

    yaourt -S broadcom-wl

**Wifi Menu  (Terminal)**

    sudo wifi-menu

**Network Manager**

    sudo pacman -S networkmanager
    sudo pacman -S network-manager-applet
    sudo pacman -S networkmanager-openvpn
    sudo pacman -S gnome-keyring

    systemctl enable NetworkManager
    systemctl start NetworkManager

**Start Network Manager Applet (startup)**

    nm-applet --sm-disable

**SSH**

    sudo pacman -S openssh
    sudo systemctl enable sshd
    sudo systemctl start sshd



## Power

**Enable MBP Fan Usage**

    yaourt -S mbpfan-git
    sudo systemctl enable mbpfan
    sudo systemctl start mbpfan

**Enable Automated Power Regulation**

    sudo pacman -S tlp
    sudo systemctl enable tlp

**Enable thermald (overheat shutoff)**

    yaourt -S thermald
    sudo systemctl enable thermald
    sudo systemctl start thermald

**Install Powertop**

    sudo pacman -S powertop
    sudo vim /etc/systemd/system/powertop.service

    #[Unit]
    #Description=Powertop tunings

    #[Service]
    #Type=oneshot
    #ExecStart=/usr/bin/powertop --auto-tune

    #[Install]
    #WantedBy=multi-user.target

    systemctl enable powertop



## Printing

**Install CUPS + HP Drivers**

    sudo pacman -S cups ghostscript gsfonts hplip
    yaourt -S libcups

**Enable CUPS Services**

    systemctl enable org.cups.cupsd.service
    systemctl start org.cups.cupsd.service

    systemctl enable org.cups.cupsd-browsed.service
    systemctl start org.cups.cupsd-browsed.service

**Access Web Interface**

    firefox http://localhost:631/



## Random

**Increase FS Watchers and Instances (Dropbox)**

    sudo echo 'fs.inotify.max_user_watches = 1048576' >> /usr/lib/sysctl.d/50-default.conf
    sudo echo 'fs.inotify.max_user_instances = 256' >> /usr/lib/sysctl.d/50-default.conf

**Ranger Configuration**

    ranger --copy-config=all

- Ascii Art Preview

        sudo pacman -S libcaca

- Foxit Reader (default pdf viewer)

        vim ~/.config/ranger/rifle.conf
        # ext pdf = foxitreader "$@"




## Appearance

**Awesome Themes**

    git clone https://github.com/mikar/awesome-themes.git
    git clone https://github.com/copycat-killer/awesome-copycats.git

**XFCE4 Windows Settings (background)**

    xfsettingsd

**Compositor**

    yaourt -S compton

**Start Compositor (background startup)**

    compton -b



## HiDPI

**XFCE**

    xfce4-appearance-settings
    # Fonts -> Custom DPI Setting -> 130

**Spotify**

    sudo vim /usr/bin/spotify
    # add: --force-device-scale-factor=1.5

**Xresources**

    vim .Xresources
    # Xft.dpi: 120

**QT**

    export QT_DEVICE_PIXEL_RATIO=1.2

**Firefox / Thunderbird**

    # about:config
    layout.css.devPixelsPerPx = 1.5



## Xmonad Startup

**Set Desktop Wallpaper**

    feh --bg-scale /path/to/wallpaper.jpg

**Initialize Cursor**

    xsetroot -cursor_name left_ptr

**Start Trayer (System Tray)**

    trayer --edge top --align right --SetDockType true --SetPartialStrut true --expand true --width 10 --height 27 --transparent true --alpha 0 --tint 0x242424



## Packages


#### Installing

Install from Arch Repository

    sudo pacman -S <package-name>

Install from Arch User Repository (AUR)

    yaourt -S <package-name>


#### Updating Packages / Repositories

Update from Arch Repository

    sudo pacman -Syu

Update from Arch User Repository

    sudo yaourt -Syu --aur


#### GUI

- **URxvt (Default terminal):** ```rxvt-unicode urxvt-perls```
- **Firefox (Developer):** ```firefox-developer (aur)```
- **Sublime Text 3:** ```sublim-text-dev (aur)```
- **Google Chrome:** ```google-chrome (aur)```
- **Dropbox:** ```dropbox (aur)```
- **Thunar-Dropbox:** ```thunar-dropbox (aur)```
- **Telegram:** ```telegram-desktop-bin (aur)```
- **Slack:** ```slack-desktop (aur)```
- **Steam:** ```steam (aur)```
- **Termite (terminal):** ```termite```
- **Spotify:** ```spotify (aur)```
- **Datagrip:** ```datagrip (aur)```
- **Parcellite (Clipboart Manager):** ```parcellite```
- **VLC:** ```vlc```
- **Feh (lightweight image viewer):** ```feh```
- **XArchiver:** ```xarchiver```
- **Meld:** ```meld```
- **Deepin Screenshot:** ```deepin-screenshot```
- **rofi (awesome launcher):** ```rofi```
- **Zoom (meetings):** ```zoom (aur)```
- **Zeal (Dash clone):** ```zeal (aur)```
- **Hexchat (IRC):** ```hexchat```
- **Geary (email):** ```geary```
- **Corebird (twitter):** ```corebird```
- **Transmission (torrent):** ```transmission-gtk```
- **Inkscape (vector art):** ```inkscape```
- **Gimp:** ```gimp```
- **Libre Office:** ```libreoffice-fresh```
- **Open Broadcaster:** ```obs-studio (aur)```
- **VirtualBox:** ```virtualbox (aur)```
    - ```sudo modprobe vboxdrv```


#### CLI

- **Archey (System Info):** ```archey-plus (aur)```
- **Speedtest CLI:** ```speedtest-cli (aur)```
- **htop:** ```htop```
- **mlocate:** ```mlocate```
    - ```sudo updatedb  # create search database```
    - ```locate file  # searches filesystem for 'file'```
- **tree:** ```tree```
- **vnstat:** ```vnstat```
- **z:** ```z-git (aur)```
- **mutt:** ```mutt```5
- **gcalcli (GCal):** ```gcalcli (aur)```
- **ranger (file browser):** ```ranger```
- **wavemon (wifi analyzer):** ```wavemon```
- **cli-visualizer (audio visualizer):** ```cli-visualizer (aur)```
- **imagemagick:** ```imagemagick```
    - ```convert file1.png file2.png file3.png +append combined.png  # concatenate three images```
- **jq (json cli tool):** ```jq```
    - ```cat file.json | jq '.'  # format json file```
- **mtr:** ```mtr```
- **ncmpcpp (cli mpd music player client):** ```ncmpcpp```
- **nmap (network scanner):** ```nmap```
    - ```nmap 10.0.1.0/24  # Scan 256 address beginning with 10.0.1.0```
- **w3m (browser):** ```w3m```


#### Other

- **Meslo Font:** ```ttf-meslo (aur)```
- **Java Web Plugin:** ```icedtea-web```
- **Flash Plugin:** ```flashplugin```
- **Flat Studio GTK:** ```gtk-theme-flatstudio (aur)```
- **Trayer SRG:** ```trayer-srg (aur)```
- **Mopidy + Spotify (Music Daemon):** ```mopidy mopidy-spotify (aur)```



## Keybindings

**Window Mgmt (Xmonad - Custom)**

- Switch Workspace: ```<cmd> + <l/r arrow>```
- Move App Workspace: ```<cmd> + <shift> + <l/r arrow>```
- Switch Screen: ```<cmd> + <u/d arrow>```
- Move App Screen: ```<cmd> + <shift> + <u/d arrow>```
- Switch Pane: ```<cmd> + j/k```
- Move App Pane: ```<cmd> + <shift> + j/k```
- Switch n Workspace: ```<cmd> + n```
- Switch Screen (1/2/3): ```<cmd> + w/e/r```
- Cycle Layout: ```<cmd> + <space>```
- Adjust Layout Master Pane Size: ```<cmd> + h/l```
- Adjust Master Pane Include Count ```<cmd> + ,/.```
- Move Selected Pane to Master: ```<cmd> + <enter>```
- Rofi App Launcher: ```<cmd> + p```


**Window Mgmt (awesome)**

- Client == Window
- Tag == Screen
- Switch Tag (Screen): ```<cmd> + <arrow>```
- Switch Previous Client (Window): ```<cmd> + j```
- Switch Next Client (Window): ```<cmd> + k```
    + Move Client: add <shift> modifier
- Toggle Client Floating: ```<cmd> + <ctrl> + <space>```
- Maximize Client: ```<cmd> + m```
- Minimize Client: ```<cmd> + n```
- Fullscreen Client: ```<cmd> + f```
- Restore All Minimized Clients: ```<cmd> + <ctrl> + n```
- Switch Layout: ```<cmd> + <space>```
- Move Client to Tag: ```<cmd> + <shift> + <num>```

**Editing**

- Home: ```<fn> + <left arrow>```
- End: ```<fn> + <right arrow>```
- Page Up: ```<fn> + <up arrow>```
- Page Down: ```<fn> + <down arrow>```
- Beginning: ```<fn> + <ctrl> + <left arrow>```
- End: ```<fn> + <ctrl> + <right arrow>```
- Next Tab: ```<fn> + <ctrl> + <down arrow>```
- Prev Tab: ```<fn> + <ctrl> + <up arrow>```
