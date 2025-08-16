source ./colors.sh

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

# Function to install packages using Chocolatey on Windows
install_with_choco() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using Chocolatey...${textreset}"
        choco install -y "$package"
    done
}

# Function to install packages using Homebrew on macOS
install_with_brew() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using Homebrew...${textreset}"
        brew install "$package"
    done
}

# Function to install packages using apt-get on Ubuntu
install_with_apt() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using apt-get...${textreset}"
        sudo apt-get install -y "$package"
    done
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

# Function to install packages using dnf on Fedora 22+ and RHEL 8+
install_with_dnf() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using dnf...${textreset}"
        sudo dnf install -y "$package"
    done
}

# Function to install packages using yum on Fedora 21, RHEL 7 and below
install_with_yum() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using yum...${textreset}"
        sudo yum install -y "$package"
    done
}

# Function to install packages using zypper on openSUSE
install_with_zypper() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using zypper...${textreset}"
        sudo zypper install -y "$package"
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

