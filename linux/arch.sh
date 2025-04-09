#!/bin/bash

# Define colors
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

cleanup_temp() {
    # Clean up temporary directories
    rm -rf "$1"
}

check_os() {
    # Check operating system
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "Linux"
        return 0
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "MacOS"
        return 0
    elif [[ "$OSTYPE" == "cygwin" || "$OSTYPE" == "msys" ]]; then
        echo "Windows"
        return 0
    fi

    echo "Unknown"
    return 1
}

# Note to self - modify this section as not working on all systems
check_distro() {
    # Check for lsb_release command
    if command -v lsb_release > /dev/null; then
       echo $(lsb_release -si)
       return 0
    fi
    # Check for /etc/os-release file
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo $NAME
        return 0
     fi

    echo "Unknown"
    return 1
}

check_package_man() {
    # Check for common package managers
    if command -v apt-get > /dev/null; then
        echo "apt-get"
        return 0
    elif command -v dnf > /dev/null; then
        echo "dnf"
        return 0
    elif command -v yum > /dev/null; then
        echo "yum"
        return 0
    elif command -v zypper > /dev/null; then
        echo "zypper"
        return 0
    elif command -v yay > /dev/null; then
        echo "yay"
        return 0
    elif command -v pacman > /dev/null; then
        echo "pacman"
        return 0
    elif command -v brew > /dev/null; then
        echo "homebrew"
        return 0
    elif command -v choco > /dev/null; then
        echo "chocolatey"
        return 0
    fi

    return 1
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

install_homebrew() {
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    (echo; echo 'eval "$(/opt/homebrew/bin/brew shellenv)"') >> $HOME/.zprofile
    eval "$(/opt/homebrew/bin/brew shellenv)"
}

install_yay() {
    install_with_pacman git
    cd /opt
    sudo git clone https://aur.archlinux.org/yay-git.git
    sudo chown -R $USER:$USER ./yay-git
    cd yay-git
    makepkg -si
    sudo yay -Syu
}

answer_default_n() {
    answer="$1"
    if [[ $answer == "Y" || $answer == "y" ]]; then
        return 0
    fi

    return 1
}

answer_default_y() {
    answer="$1"
    if answer_default_n "$answer" || [[ $answer == "" ]]; then
        return 0
    fi
    
    return 1
}


# Function to open a URL in the default browser
open_url() {
  local url=$1
  # Check if xdg-open is available
  if command -v xdg-open >/dev/null; then
    xdg-open "$url"
    return 0
  fi

  echo "Error: Unable to open URL. No supported command found."
  return 1
}

add_nerd_font() {
    fontName=$1
    # Define the download URL for the Nerd Font ZIP file
    fontUrl="https://github.com/ryanoasis/nerd-fonts/releases/latest/download/$fontName.zip"
    # Create a temporary directory to extract the font files
    tempDir=$(mktemp -d)

    trap 'cleanup_temp "$tempDir"' EXIT

    # Download the Nerd Font ZIP file
    curl -L -o "$tempDir/$fontName.zip" "$fontUrl"

    # Extract the font files from the ZIP archive
    unzip -q "$tempDir/$fontName.zip" -d "$tempDir"

    # Install the Nerd Font
    find "$tempDir" -name '*.ttf' -exec cp {} ~/.local/share/fonts/ \;

    # Refresh the font cache
    fc-cache -f -v

    echo "Nerd Font '$fontName' installed successfully."

}


# install shells
install_with_pacman zsh fish neovim git gnome xorg xorg-server

install_yay

sudo systemctl start gdm
sudo systemctl enable gdm4r

sudo pacman -S gnome-tweaks