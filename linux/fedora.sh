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

# Variables
categories=(
    "All"
    "Fonts"
    "Package Managers"
    "Shells"
    "Programming Languages"
    "Browsers"
    "CLI Tools"
    "Customization"
    "Communication"
    "Creative Tools"
    "Dev Tools"
    "IDEs"
    "Video Players"
    "Audio Tools"
    "Productivity"
    "Virtualization & Containers"
    "Networking"
    "Gaming & Game Development"
    "Security & Privacy"
    "Streaming & Recording"
    "Utilities"
    "Customization & Theming"
    "Done"
)

nerd_font_list=(
    "3270"
    "0xProto"
    "Agave"
    "AnonymousPro"
    "Arimo"
    "AurulentSansMono"
    "BigBlueTerminal"
    "BitstreamVeraSansMono"
    "CascadiaCode"
    "CascadiaMono"
    "CodeNewRoman"
    "ComicShannsMono"
    "CommitMono"
    "Cousine"
    "D2Coding"
    "DaddyTimeMono"
    "DejaVuSansMono"
    "DepartureMono"
    "DroidSansMono"
    "EnvyCodeR"
    "FantasqueSansMono"
    "FiraCode"
    "FiraMono"
    "FontPatcher"
    "GeistMono"
    "Go-Mono"
    "Gohu"
    "Hack"
    "Hasklig"
    "HeavyData"
    "Hermit"
    "iA-Writer"
    "IBMPlexMono"
    "Inconsolata"
    "InconsolataGo"
    "InconsolataLGC"
    "IntelOneMono"
    "Iosevka"
    "IosevkaTerm"
    "IosevkaTermSlab"
    "JetBrainsMono"
    "Lekton"
    "LiberationMono"
    "Lilex"
    "MartianMono"
    "Meslo"
    "Monaspace"
    "Monofur"
    "Monoid"
    "Mononoki"
    "MPlus"
    "NerdFontsSymbolsOnly"
    "Noto"
    "OpenDyslexic"
    "Overpass"
    "ProFont"
    "ProggyClean"
    "Recursive"
    "RobotoMono"
    "ShareTechMono"
    "SourceCodePro"
    "SpaceMono"
    "Terminus"
    "Tinos"
    "Ubuntu"
    "UbuntuMono"
    "UbuntuSans"
    "VictorMono"
    "ZedMono"
)

# Functions
is_sudo() {
    if sudo -n true 2>/dev/null; then
        return 0
    fi
    return 1
}

check_arch() {
    arch=$(uname -m)
    if [[ "$arch" == "aarch64" ]]; then
        arch="ARM64"
    elif [[ "$arch" == "x86_64" ]]; then
        arch="x64"
    elif [[ "$arch" == *"arm"* ]]; then
        arch="ARM"
    else
        arch="Unknown"
    fi

    echo $arch
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

open_url() {
    local url=$1
    # Check if xdg-open is available (Linux only)
    if command -v xdg-open >/dev/null; then
        xdg-open "$url"
        return 0
    fi
    return 1
}

install_nerd_font() {
    local font="$1"
    local fonts_dir="$HOME/.local/share/fonts"
    local font_dir="$fonts_dir/$font"
    local font_version="v3.3.0"
    local font_extension="zip"
    # check if the font exists in the nerd_font_list array
    if [[ ! " ${nerd_font_list[@]} " =~ " ${font} " ]]; then
        echo -e "${textred}Error: Invalid font name. Please choose from the following:${textreset}"
        echo -e "${textyellow}${nerd_font_list[*]}${textreset}"
        return 1
    fi

    # Let user know what's happening
    echo -e "${textgreen}Installing font $font...${textreset}"
    # check if -d flag is set
    if [[ $2 == "-d" ]]; then
        auto_install=true
    else
        # Ask if the user wants to install the font as zip or tar.xz
        echo -e "${textgreen}Do you want to install the font as a zip or tar.xz file (Default is zip)?${textreset}"
        echo -e "${textyellow}1) zip${textreset}"
        echo -e "${textyellow}2) tar.xz${textreset}"
        read -p "" answer
        if [[ $answer == "1" ]]; then
            font_extension="zip"
        elif [[ $answer == "2" ]]; then
            font_extension="tar.xz"
        fi
    fi

    font_url="https://github.com/ryanoasis/nerd-fonts/releases/download/$font_version/$font.$font_extension"
    # Create a temporary directory to download the font
    temp_dir=$(mktemp -d)
    # Create the font directory if it doesn't exist
    mkdir -p "$font_dir"
    # Download the font
    echo -e "${textgreen}Downloading $font...${textreset}"
    wget -q --show-progress -P "$temp_dir" "$font_url"
    # Extract the font files from the ZIP archive
    echo -e "${textgreen}Extracting $font...${textreset}"
    if [[ $font_extension == "zip" ]]; then
        unzip -q "$temp_dir/$font.$font_extension" -d "$temp_dir"
    elif [[ $font_extension == "tar.xz" ]]; then
        tar -xf "$temp_dir/$font.$font_extension" -C "$temp_dir"
    fi

    # Install the font
    echo -e "${textgreen}Installing $font...${textreset}"
    # copy all files from temp dir to fonts directory
    find "$temp_dir" -name '*.ttf' -exec cp {} "$font_dir" \;
    # cleanup
    ls -la "$temp_dir"
    rm -rf "$temp_dir"
    echo -e "${textgreen}$font installed.${textreset}"
    return 0
    
}

install_nerd_fonts() {
    # Ask the user to select a font
    echo -e "${textgreen}Please select a font to install:${textreset}"
    selected_fonts=()
    available_fonts=(
        "All" 
        "${nerd_font_list[@]}" 
        "Done"
    )

    # show the list of available fonts
    while true; do
        echo "Available Fonts:"
        select opt in "${available_fonts[@]}"; do
            if [[ "$opt" == "Done" ]]; then
                break 2  # Exit both loops
            elif [[ "$opt" == "All" ]]; then
                selected_fonts=("${nerd_font_list[@]}")
                break 2  # Exit both loops
            elif [[ -n "$opt" ]]; then
                if [[ ! " ${selected_fonts[@]} " =~ " ${opt} " ]]; then
                    selected_fonts+=("$opt")
                    echo -e "${textgreen}✔ Added: $opt${textreset}"
                else
                    echo -e "${textyellow}⚠ $opt is already selected!${textreset}"
                fi
                break
            else
                echo -e "${textred}❌ Invalid choice, please try again.${textreset}"
            fi
        done
    done

    # Execute installations
    echo -e "${textblue}Starting installation process...${textreset}"
    for font in "${selected_fonts[@]}"; do
        install_nerd_font "$font" -d
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

install_nix() {
    if command -v nix-env >/dev/null; then
        echo -e "${textgreen}NIX is already installed.${textreset}"
        return 0
    fi
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
    if command -v flatpak >/dev/null; then
        echo -e "${textgreen}Flatpak is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing Flatpak...${textreset}"
    install_with_dnf flatpak
    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    echo -e "${textgreen}Flatpak installed.${textreset}"
}

install_with_flatpak() {
    for package in "$@"; do
        echo -e "${textgreen}Installing $package using flatpak...${textreset}"
        flatpak install flathub "$package"
    done
}

# install package manager
install_package_manager() {
    # Would you like to install Nix?
    read -p -e "${textgreen}Would you like to install Nix? [Y/n] ${textreset}" answer
    if answer_default_y "$answer"; then
        install_nix
    fi

    # Would you like to install Flatpak?
    echo -e "${textgreen}Would you like to install Flatpak? [Y/n] ${textreset}"
    if answer_default_y "$answer"; then
        install_flatpak
    fi

    # Would you like to install Homebrew?
    echo -e "${textgreen}Would you like to install Homebrew? [Y/n] ${textreset}"
    if answer_default_y "$answer"; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi

    # Would you like to install nodeJS?
    echo -e "${textgreen}Would you like to install nodeJS? [Y/n] ${textreset}"
    if answer_default_y "$answer"; then
        install_with_dnf nodejs
    fi

    # Would you like to install NPM?
    echo -e "${textgreen}Would you like to install NPM? [Y/n] ${textreset}"
    if answer_default_y "$answer"; then
        install_with_dnf npm
    fi

    # Would you like to install Yarn?
    echo -e "${textgreen}Would you like to install Yarn? [Y/n] ${textreset}"
    if answer_default_y "$answer"; then
        install_with_dnf yarn
    fi

    # Would you like to install Composer?
    echo -e "${textgreen}Would you like to install Composer? [Y/n] ${textreset}"
    if answer_default_y "$answer"; then
        install_with_dnf composer
    fi
}

# Install fish
install_fish() {
    if command -v fish >/dev/null; then
        echo -e "${textgreen}Fish is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing Fish...${textreset}"
    install_with_dnf fish
    echo -e "${textgreen}Fish installed.${textreset}"
}
# Install Zsh
install_zsh() {
    if command -v zsh >/dev/null; then
        echo -e "${textgreen}Zsh is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing Zsh...${textreset}"
    install_with_dnf zsh
    echo -e "${textgreen}Zsh installed.${textreset}"
}

# Install Oh My Zsh
install_oh_my_zsh() {
    if [ -d "$HOME/.oh-my-zsh" ]; then
        echo -e "${textgreen}Oh My Zsh is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing Oh My Zsh...${textreset}"
    sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
    echo -e "${textgreen}Oh My Zsh installed.${textreset}"
}

# Install Shell
install_shell() {
    read -p "Do you want to install an additional shell? [Y/n] " answer
    if answer_default_y "$answer"; then
        read -p "Do you want to install the zsh shell? [Y/n] " answer
        if answer_default_y "$answer"; then
            install_zsh
        fi
        read -p "Do you want to install the fish shell? [Y/n] " answer
        if answer_default_y "$answer"; then
            install_fish
        fi
    fi
}

# Install Programming Languages
install_go() {
    if command -v go >/dev/null; then
        echo -e "${textgreen}Go is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing Go...${textreset}"
    install_with_dnf golang
    echo -e "${textgreen}Go installed.${textreset}"
}

install_php() {
    if command -v php >/dev/null; then
        echo -e "${textgreen}PHP is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing PHP...${textreset}"
    install_with_dnf php
    echo -e "${textgreen}PHP installed.${textreset}"
}

install_dotnet() {
    if command -v dotnet >/dev/null; then
        echo -e "${textgreen}.NET is already installed.${textreset}"
        return 0
    fi
    echo -e "${textgreen}Installing .NET...${textreset}"
    install_with_dnf dotnet
    echo -e "${textgreen}.NET installed.${textreset}"
}

# Define available installation categories
install_fonts() {
    echo -e "${textblue}Installing fonts...${textreset}"
    install_nerd_fonts
}

install_package_managers() {
    echo -e "${textblue}Installing package managers...${textreset}"
    install_package_manager
}

install_shells() {
    echo -e "${textblue}Installing alternative shells...${textreset}"
    install_shell
}

install_programming_languages() {
    echo -e "${textblue}Installing programming languages...${textreset}"
    install_go
    install_php
}

install_browsers() {
    echo -e "${textblue}Installing browsers...${textreset}"
    install_with_dnf firefox
}

install_cli_tools() {
    echo -e "${textblue}Installing CLI utilities...${textreset}"
    install_with_dnf jq
}

install_customization() {
    echo -e "${textblue}Installing customization tools...${textreset}"
    # Add customization installation commands here
}

install_communication() {
    echo -e "${textblue}Installing communication apps...${textreset}"
    install_with_flatpak slack
}

install_creative_tools() {
    echo -e "${textblue}Installing creative software...${textreset}"
    install_with_flatpak gimp
}

install_dev_tools() {
    echo -e "${textblue}Installing development tools...${textreset}"
    install_with_dnf git
}

install_ides() {
    echo -e "${textblue}Installing IDEs...${textreset}"
    install_with_flatpak com.visualstudio.code
}

install_video_players() {
    echo -e "${textblue}Installing video players...${textreset}"
    install_with_flatpak org.videolan.VLC
}

install_audio_tools() {
    echo -e "${textblue}Installing audio production tools...${textreset}"
    install_with_flatpak org.audacityteam.Audacity
}

install_productivity() {
    echo -e "${textblue}Installing productivity applications...${textreset}"
    install_with_flatpak org.onlyoffice.desktopeditors
}

install_virtualization_containers() {
    echo -e "${textblue}Installing virtualization software...${textreset}"
    install_with_dnf virtualbox
}

install_networking() {
    echo -e "${textblue}Installing networking tools...${textreset}"
    install_with_dnf wireshark
}

install_gaming_game_development() {
    echo -e "${textblue}Installing game development software...${textreset}"
    install_with_flatpak com.unity.UnityHub
}

install_security_privacy() {
    echo -e "${textblue}Installing security and privacy tools...${textreset}"
    install_with_dnf keepassxc
}

install_streaming_recording() {
    echo -e "${textblue}Installing streaming and recording software...${textreset}"
    install_with_flatpak com.obsproject.Studio
}

install_utilities() {
    echo -e "${textblue}Installing system utilities...${textreset}"
    install_with_dnf htop
}

install_customization_theming() {
    echo -e "${textblue}Installing Customization Tools...${textreset}"
    install_oh_my_zsh
}

# Install all available software
install_all() {
    echo -e "${textblue}Installing all available software...${textreset}"
    install_fonts
    install_package_managers
    install_shells
    install_programming_languages
    install_browsers
    install_cli_tools
    install_customization
    install_communication
    install_creative_tools
    install_dev_tools
    install_ides
    install_video_players
    install_audio_tools
    install_productivity
    install_virtualization_containers
    install_networking
    install_gaming_game_development
    install_security_privacy
    install_streaming_recording
    install_utilities
    install_customization_theming
}

install_category() {
    case "$1" in
        "All") install_all ;;
        "Fonts") install_fonts ;;
        "Package Managers") install_package_managers ;;
        "Shells") install_shells ;;
        "Programming Languages") install_programming_languages ;;
        "Browsers") install_browsers ;;
        "CLI Tools") install_cli_tools ;;
        "Customization") install_customization ;;
        "Communication") install_communication ;;
        "Creative Tools") install_creative_tools ;;
        "Dev Tools") install_dev_tools ;;
        "IDEs") install_ides ;;
        "Video Players") install_video_players ;;
        "Audio Tools") install_audio_tools ;;
        "Productivity") install_productivity ;;
        "Virtualization & Containers") install_virtualization_containers ;;
        "Networking") install_networking ;;
        "Gaming & Game Development") install_gaming_game_development ;;
        "Security & Privacy") install_security_privacy ;;
        "Streaming & Recording") install_streaming_recording ;;
        "Utilities") install_utilities ;;
        "Customization & Theming") install_customization_theming ;;
        "Done") return 1 ;;
        *) echo -e "${textred}Invalid category: $1${textreset}" ;;
    esac
}

start() {
    PS3="Select a category (or choose 'Done' to start installation): "
    selected_categories=()

    while true; do
        echo "Available Categories:"
        select opt in "${categories[@]}"; do
            if [[ "$opt" == "Done" || "$opt" == "All" ]]; then
                break 2  # Exit both loops
            elif [[ -n "$opt" ]]; then
                if [[ ! " ${selected_categories[@]} " =~ " ${opt} " ]]; then
                    selected_categories+=("$opt")
                    echo -e "${textgreen}✔ Added: $opt${textreset}"
                else
                    echo -e "${textyellow}⚠ $opt is already selected!${textreset}"
                fi
                break
            else
                echo -e "${textred}❌ Invalid choice, please try again.${textreset}"
            fi
        done
    done

    # Execute installations
    echo -e "${textblue}Starting installation process...${textreset}"
    for category in "${selected_categories[@]}"; do
        install_category "$category"
    done

    echo -e "${textgreen}✅ Installation completed for: ${selected_categories[*]}${textreset}"
}

start
