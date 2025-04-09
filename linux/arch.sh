#!/bin/bash

# Colors
textblack='\033[0;30m' # Black - Regular
textred="\033[0;31m" # Red
textgreen='\033[0;32m' # Green
textyellow='\033[0;33m' # Yellow
textblue='\033[0;34m' # Blue
textpurple='\033[0;35m' # Purple
textcyan='\033[0;36m' # Cyan
textwhite="\033[0;37m" # White
textreset='\033[0m'

# Functions
is_sudo() {
    if sudo -n true 2>/dev/null; then
        echo true
    else
        echo false
    fi
}

check_arch() {
    arch=$(uname -m)
    if [[ "$arch" == "x86_64" ]]; then
        arch="x64"
    elif [[ "$arch" == *"arm"* ]]; then
        arch="ARM"
    else
        arch="Unknown"
    fi

    echo $arch
}

# Function to install packages using yay on Arch Linux
install_with_yay() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using pacman...${textreset}"
        sudo yay -S --noconfirm "$package"
    done
}

# Function to install packages using pacman on Arch Linux
install_with_pacman() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using pacman...${textreset}"
        sudo pacman -S --noconfirm "$package"
    done
}

install_nix() {
    echo -e "${textgreen}Installing Nix...${textreset}"
    sh <(curl -L https://nixos.org/nix/install) --daemon
    echo -e "${textgreen}Nix installed.${textreset}"
}

install_with_nix() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using nix...${textreset}"
        nix-env -i "$package"
    done
}

install_flatpak() {
    echo -e "${textgreen}Installing Flatpak...${textreset}"
    install_with_pacman flatpak
    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    echo -e "${textgreen}Flatpak installed.${textreset}"
}

install_with_flatpak() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using flatpak...${textreset}"
        flatpak install flathub "$package"
    done
}

# install shells
install_with_pacman zsh fish neovim git gnome xorg xorg-server

install_yay

sudo systemctl start gdm
sudo systemctl enable gdm4r

sudo pacman -S gnome-tweaks