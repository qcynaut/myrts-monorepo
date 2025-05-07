#!/bin/bash

RED='\033[0;31m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
NC='\033[0m'

# print with color given if stdout is a tty
print_color() {
    if [ -t 1 ]; then
        echo -e "$1$2$NC"
    else
        echo "$2"
    fi
}

# print with red color
error() {
    print_color "$RED" "$@"
}

# print with green color
success() {
    print_color "$GREEN" "$@"
}

# print with yellow color
info() {
    print_color "$YELLOW" "$@"
}

# print ascii gravity
myrts() {
    success "
  __  ____     _______ _______ _____ 
 |  \/  \ \   / /  __ \__   __/ ____|
 | \  / |\ \_/ /| |__) | | | | (___  
 | |\/| | \   / |  _  /  | |  \___ \ 
 | |  | |  | |  | | \ \  | |  ____) |
 |_|  |_|  |_|  |_|  \_\ |_| |_____/ 
                                     
                                     

  @copyright 2023 myrts
    "
}

# run command and exit on error
cmd() {
    res=$(eval "$@" 2>/dev/null)
    if [ $? -ne 0 ]; then
        error "Error when running command: $@"
        exit 1
    fi
}

# check os and arch
chkos() {
    cmd "uname -s"
    os=$res
    cmd "uname -m"
    arch=$res
    if [ "$os" == "Linux" ]; then
        if [ "$arch" != "aarch64" ]; then
            error "Unsupported architecture: $arch"
            exit 1
        fi
    else
        error "Unsupported OS: $os"
        exit 1
    fi
}

# check root
chkroot() {
    if [ "$EUID" -ne 0 ]; then
        error "Please run as root"
        exit 1
    fi
}

# check service
check_service() {
    if service --status-all | grep -Fq "$1"; then
        res=0
    else
        res=1
    fi
}

# get input from user
get_input() {
    exec 8<&1
    read -u 8 -p "$1" res
    exec 8<&-
}

# prompt user to confirm
confirm() {
    get_input "$1 [y/n]  "
    if [ "$res" == "y" ] || [ "$res" == "Y" ]; then
        res=0
    else
        res=1
    fi
}

# remove file if it exists
rm_file() {
    if [ -f "$1" ]; then
        rm "$1"
    fi
}

# remove directory if it exists
rm_dir() {
    if [ -d "$1" ]; then
        rm -rf "$1"
    fi
}

# download necessary packages
download() {
    apt update -y && sudo apt upgrade -y
    apt install -y curl pulseaudio libpulse-dev librust-libpulse-sys-dev alsa-utils libogg-dev libvorbis-dev ffmpeg libopus-dev libasound2-dev libudev-dev libopusfile-dev llvm-dev libclang-dev clang openssl libssl-dev libavutil-dev libavcodec-dev libavformat-dev

    success "Extra packages installed successfully"
    info "Clean up..."
    sleep 2
    clear
}

# the main function
main() {
    myrts
    # sleep for 2 seconds
    sleep 2
    
    get_input "masukan alamat:  "
    alamat=$res
    get_input "masukan deskripsi:  "
    deskripsi=$res
    info "Checking root..."
    chkroot
    info "Checking os..."
    chkos
    info "Checking service..."
    check_service "myrts-client"
    info "myrts-client is already installed, removing..."
    systemctl stop myrts-client
    systemctl disable myrts-client
    systemctl daemon-reload
    rm_file "/etc/systemd/system/myrts-client.service"
    rm_dir "/var/lib/myrts"

    info "Downloading extra packages..."

    download
    
    base_url="https://api.myrts.qcynaut.tech/install"

    info "Configuring asound..."
    curl -L "$base_url/asound.state" -o /tmp/asound.state
    cmd "alsactl --file /tmp/asound.state restore 0 -U"

    info "Configuring alsa..."
    curl -L "$base_url/asound.conf" -o /etc/asound.conf

    info "Installing myrts-client..."
    url="$base_url/myrts-client-dev.service?address=$alamat&description=$deskripsi"
    curl -L $url -o /etc/systemd/system/myrts-client.service
    mkdir -p /var/lib/myrts
    curl -L "$base_url/myrts-client" -o /var/lib/myrts/myrts-client
    chmod +x /var/lib/myrts/myrts-client

    info "Starting myrts-client..."
    systemctl start myrts-client
    systemctl enable myrts-client

    success "Installed successfully"
    confirm "Setup interface?"
    if [ "$res" -eq 0 ]; then
        setup_interface
    fi
    monitor
}

monitor() {
    confirm "Do you want to monitor myrts-client?"
    if [ "$res" -eq 0 ]; then
        info "Monitoring myrts-client..."
        tail -f /var/lib/myrts/logs/myrts-client.log
    fi
}

setup_interface() {
    clear
    get_input "masukan ip address:  "
    ip_address=$res
    get_input "masukan netmask:  "
    netmask=$res
    get_input "masukan gateway:  "
    gateway=$res
    echo "source /etc/network/interfaces.d/*
# Network is managed by Network manager
auto lo
iface lo inet loopback
auto end0
allow-hotplug end0
iface end0 inet static
    address $ip_address
    netmask $netmask
    gateway $gateway
    dns-nameservers 1.1.1.1 8.8.8.8
" > /etc/network/interfaces
    success "Interface setup successfully"
}

main