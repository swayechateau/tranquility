#!/bin/sh

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

check_brew() {
    if command -v brew > /dev/null; then
        echo "Homebrew is installed"
    else
    # Install Homebrew
        echo "Homebrew might be missing from path... reinstalling"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
}

install_with_brew() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using Homebrew...${textreset}"
        brew install "$package"
    done
}


# Install Xcode Cli
read -p "Do you want to install xcode command line tools? [Y/n] " answer
if answer_default_y "$answer"; then
    echo "After installing the command line tools, please install xcode from the app store."
    xcode-select --install
fi

# Install shell
read -p "Do you want to install an additional shell? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install the fish shell? [Y/n] " answer
    if answer_default_y "$answer"; then
        echo "Installing fish..."
        brew install fish
    fi
fi