#!/bin/sh

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

cleanup_temp() {
    # Clean up temporary directories
    rm -rf "$1"
}


check_package_man() {
    # Check for common package managers
    if command -v brew > /dev/null; then
        echo "homebrew installed"
        return 0
    fi

    return 1
}


# Function to install packages using Homebrew on macOS
install_with_brew() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using Homebrew...${textreset}"
        brew install "$package"
    done
}

install_homebrew() {
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    (echo; echo 'eval "$(/opt/homebrew/bin/brew shellenv)"') >> $HOME/.zprofile
    eval "$(/opt/homebrew/bin/brew shellenv)"
}

install_chocolatey() {
    echo "Installing Chocolatey"
    powershell -Command "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))"
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

install_package_manager() {
  os=$(check_os)
  if [[ $os == "Windows" ]]; then
    # If os is windows and pkg is null then install Chocolatey
    echo "No package manager detected. Installing Chocolatey."
    install_chocolatey
    return 0
  elif [[ $os == "macOS" ]]; then
    # If os is mac and pkg is null then install Homebrew
    echo "No package manager detected. Installing Homebrew."
    install_homebrew
    return 0
  elif [[ $os == "Linux" ]]; then
    # If os is linux and pkg is null then install Homebrew
    echo "No package manager detected. Installing Homebrew."
    install_homebrew
    return 0
  fi

  echo "Error: Unable to assertain Operating System."
  return 1

}

pkg_man_install() {
    pkg_man=$(check_package_man)
    
    if ! $pkg_man; then
        echo -e "${textred}No package manager found.${textreset}"
        exit 1
    fi

    for package in "$@"; do
        echo -e "${textgreen}Installing $package using $pkg_man...${textreset}"
        if [[ $pkg_man == "chocolatey" ]]; then
            install_with_choco "$package"
        elif [[ $pkg_man == "homebrew" ]]; then
            install_with_brew "$package"
        elif [[ $pkg_man == "apt-get" ]]; then
            install_with_apt "$package"
        elif [[ $pkg_man == "pacman" ]]; then
            install_with_pacman "$package"
        elif [[ $pkg_man == "dnf" ]]; then
            install_with_dnf "$package"
        elif [[ $pkg_manager == "yum" ]]; then
            install_with_yum "$package"
        elif [[ $pkg_manager == "zypper" ]]; then
            install_with_zypper "$package"
        fi
    done
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
  # Check if macOS open command is available
  elif command -v open >/dev/null; then
    open "$url"
    return 0
  # Check if Windows start command is available through WSL
  elif command -v cmd.exe >/dev/null; then
    cmd.exe /C "start $url"
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



os=$(check_os)
pkg_man=$(check_package_man)

# OS using PKGMAN
echo "$os Detected. Checking package manager..."

# Check pacakge Manager

# Check Homebrew
if command -v brew >/dev/null 2>&1; then
    echo "Homebrew is installed"
else
# Install Homebrew
    echo "Homebrew might be missing from path... reinstalling"
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

echo "$pkg_man deteced, using for package installation."

if [[ "$os" == "MacOS" ]]; then
    # Install Xcode Cli
    read -p "Do you want to install xcode command line tools? [Y/n] " answer
    if answer_default_y "$answer"; then
        echo "After installing the command line tools, please install xcode from the app store."
        xcode-select --install
    fi

    read -p "Do you want to add fonts to homebrew? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew tap homebrew/cask-fonts
    fi
fi

if [[ "$os" == "MacOS" ]]; then
  # Install Xcode Cli
  read -p "Do you want to install xcode command line tools? [Y/n] " answer
  if answer_default_y "$answer"; then
    echo "After installing the command line tools, please install xcode from the app store."
    xcode-select --install
  fi

  read -p "Do you want to add fonts to homebrew? [Y/n] " answer
  if answer_default_y "$answer"; then
    brew tap homebrew/cask-fonts
  fi

  read -p "Do you want to install rectangle? [Y/n] " answer
  if answer_default_y "$answer"; then
    brew install rectangle
  fi
fi

# Install shell


# CLI Tools
read -p "Do you want to install CLI tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install git? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install git
    fi

    read -p "Do you want to install wget? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install wget
    fi

    read -p "Do you want to install curl? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install curl
    fi

    # Linux Only
    # read -p "Do you want to install docker and docker-compose? [Y/n] " answer
    # if answer_default_y "$answer"; then
    #     brew install docker-compose
    # fi

    read -p "Do you want to install tmux? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install tmux
    fi

    read -p "Do you want to install terraform? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install terraform
    fi

    read -p "Do you want to install awscli? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install awscli
    fi

    read -p "Do you want to install deno? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install deno
    fi

    read -p "Do you want to install nodejs and npm? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install node
    fi

    read -p "Do you want to install yarn? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install yarn
    fi

    read -p "Do you want to install jq? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install jq
    fi

    read -p "Do you want to install go? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install go
    fi

    read -p "Do you want to install dotnet-sdk? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install dotnet-sdk
    fi

    read -p "Do you want to install elixir? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install elixir
    fi

    read -p "Do you want to install python3 and pip3? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install python
    fi

    read -p "Do you want to install rust and cargo? [Y/n] " answer
    if answer_default_y "$answer"; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    fi

    read -p "Do you want to install php and composer? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install php
    fi

    read -p "Do you want to install ruby and gem? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install ruby
    fi
fi

# Editors and IDEs
read -p "Do you want to install an editor? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install neovim? [Y/n] " answer
    if answer_default_y "$answer"; then
        echo "Installing neovim..."
        brew install neovim
    fi
    read -p "Do you want to install visual studio? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask visual-studio-code
    fi
    read -p "Do you want to install a visual studio code? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask visual-studio
    fi
    read -p "Do you want to install sublime text? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask sublime-text
    fi
    read -p "Do you want to install atom? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask atom
    fi
    read -p "Do you want to install brackets? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask brackets
    fi
    read -p "Do you want to install eclipse? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask eclipse-java
    fi
    read -p "Do you want to install intellij idea community edition? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install intellij-idea-ce
    fi
fi

# Dev GUI Tools (MAC/Windows)
read -p "Do you want to install dev GUI tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install docker desktop? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask docker
        brew install orbstack
    fi
    read -p "Do you want to install github desktop? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask github
    fi
fi

# Mobile Development
read -p "Do you want to install mobile development tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install android studio? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask android-studio
    fi
    read -p "Do you want to install xcode? [Y/n] " answer
    if answer_default_y "$answer"; then
        echo "Please install xcode from the app store. https://apps.apple.com/us/app/xcode/id497799835?mt=12"
        open_url "https://apps.apple.com/us/app/xcode/id497799835?mt=12"
    fi
fi

# Virtaulization
read -p "Do you want to install virtualization tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install virtualbox? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install virtualbox
    fi

    read -p "Do you want to install vagrant? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install vagrant
    fi
fi

read -p "Do you want to install a browser? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install Brave browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        pkg_man_install brave-browser
    fi

    read -p "Do you want to install Opera browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        read -p "Install Standard Opera? [Y/n] " answer
        if answer_default_y "$answer"; then
            pkg_man_install opera
        fi

        read -p "Install OperaGX? [Y/n] " answer
        if answer_default_y "$answer"; then
            pkg_man_install opera-gx
        fi
    fi

    read -p "Do you want to install Vivaldi browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        read -p "Install Standard Vivaldi? [Y/n] " answer
        if answer_default_y "$answer"; then
            pkg_man_install vivaldi
        fi
    fi

    read -p "Do you want to install Tor browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        pkg_man_install tor-browser
    fi

    read -p "Do you want to install Google Chrome browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        read -p "Install Standard Google Chrome? [Y/n] " answer
        if answer_default_y "$answer"; then
            pkg_man_install google-chrome
        fi
    fi

    read -p "Do you want to install Microsoft Edge browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        read -p "Install Standard Microsoft Edge? [Y/n] " answer
        if answer_default_y "$answer"; then
            pkg_man_install microsoft-edge
        fi
    fi

    read -p "Do you want to install Firefox browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        read -p "Install Standard Firefox? [Y/n] " answer
        if answer_default_y "$answer"; then
            pkg_man_install firefox
        fi
    fi

    read -p "Do you want to install Polypane browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask polypane
    fi
fi

# Chat Application
read -p "Do you want to install chat applications? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install slack? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask slack
    fi

    read -p "Do you want to install discord? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask discord
    fi

    read -p "Do you want to install telegram-desktop? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask telegram-desktop
    fi

    # not on brew - need to add manually
    read -p "Do you want to install line? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask line
    fi
    # error based on ssl cert
    read -p "Do you want to install wechat? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask wechat
    fi
    # not on brew - need to add manually
    read -p "Do you want to install kakaotalk? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask kakaotalk
    fi

    # WhatsApp Desktop
    read -p "Do you want to install WhatsApp? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask whatsapp
    fi
fi

# Games Development Tools
read -p "Do you want to install games development tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install unity? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask unity
    fi
    # not on brew
    read -p "Do you want to install unreal-engine? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask unreal-engine
    fi

    read -p "Do you want to install godot? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask godot
    fi
fi


# Graphics Editor Tools
read -p "Do you want to install graphics editor tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install gimp? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask gimp
    fi

    read -p "Do you want to install inkscape? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask inkscape
    fi

    read -p "Do you want to install krita? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask krita
    fi

    read -p "Do you want to install blender? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask blender
    fi

    read -p "Do you want to install staruml? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask staruml
    fi
fi

# Video Editor Tools
read -p "Do you want to install video editor tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install obs? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask obs
    fi

    read -p "Do you want to install davinci-resolve? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask davinci-resolve
    fi

    read -p "Do you want to install handbrake? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask handbrake
    fi

    read -p "Do you want to install vlc? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask vlc
    fi
fi

# Audio Editor Tools
read -p "Do you want to install audio editor tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install audacity? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask audacity
    fi

    read -p "Do you want to install lmms? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask lmms
    fi

    read -p "Do you want to install reaper? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask reaper
    fi
fi

# Music Apps
read -p "Do you want to install music apps? [Y/n] " answer

    read -p "Do you want to install spotify? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask spotify
    fi
fi

# Database Management Tools
read -p "Do you want to install database management tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install dbeaver-community? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask dbeaver-community
    fi

    read -p "Do you want to install mongodb-compass? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask mongodb-compass
    fi
fi

# VPN Tools
read -p "Do you want to install vpn tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install surfshark? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask surfshark
    fi

    read -p "Do you want to install openvpn-connect? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask openvpn-connect
    fi
fi

# REST client tools
read -p "Do you want to install REST client tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install insomnia? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask insomnia
    fi

    read -p "Do you want to install postman? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask postman
    fi
fi

# Download Manager Tools
read -p "Do you want to install download manager tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install jdownloader? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask jdownloader
    fi
fi

# Note Taking Tools
read -p "Do you want to install note taking tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install notion? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask notion
    fi

    read -p "Do you want to install typora? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask typora
    fi

    read -p "Do you want to install obsidian? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask obsidian
    fi
fi

# Password Manager Tools
read -p "Do you want to install password manager tools? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install bitwarden? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask bitwarden
    fi

    read -p "Do you want to install keepassxc? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask keepassxc
    fi
fi

# Gaming Clients
read -p "Do you want to install gaming clients? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install steam? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask steam
    fi

    read -p "Do you want to install epic-games? [y/N] " answer
    if answer_default_n "$answer"; then
        brew install --cask epic-games
    fi

    read -p "Do you want to install gog-galaxy? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask gog-galaxy
    fi

    read -p "Do you want to install origin? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask origin
    fi

    read -p "Do you want to install battle-net? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask battle-net
    fi

    read -p "Do you want to install playstation remote-play? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask sony-ps-remote-play
    fi
fi

# Operating System Extensions
read -p "Do you want to install operating system extensions? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install copyq? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask copyq
    fi

    read -p "Do you want to install rectangle? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask rectangle
    fi

    read -p "Do you want to install iterm2? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install --cask iterm2
    fi
fi

echo "Installation complete."
