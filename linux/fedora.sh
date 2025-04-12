#!/bin/bash

# Colors
declare -A colors=(
    ["black"]='\033[0;30m'
    ["red"]='\033[0;31m'
    ["green"]='\033[0;32m'
    ["yellow"]='\033[0;33m'
    ["blue"]='\033[0;34m'
    ["purple"]='\033[0;35m'
    ["cyan"]='\033[0;36m'
    ["white"]='\033[0;37m'
    ["reset"]='\033[0m'
)

# Variables
categories=(
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
)

nerd_font_list=(
    "3270" "0xProto" "Agave" "AnonymousPro" "Arimo" "AurulentSansMono" "BigBlueTerminal" "BitstreamVeraSansMono"
    "CascadiaCode" "CascadiaMono" "CodeNewRoman" "ComicShannsMono" "CommitMono" "Cousine" "D2Coding" "DaddyTimeMono"
    "DejaVuSansMono" "DepartureMono" "DroidSansMono" "EnvyCodeR" "FantasqueSansMono" "FiraCode" "FiraMono" "FontPatcher"
    "GeistMono" "Go-Mono" "Gohu" "Hack" "Hasklig" "HeavyData" "Hermit" "iA-Writer" "IBMPlexMono" "Inconsolata" "InconsolataGo"
    "InconsolataLGC" "IntelOneMono" "Iosevka" "IosevkaTerm" "IosevkaTermSlab" "JetBrainsMono" "Lekton" "LiberationMono"
    "Lilex" "MartianMono" "Meslo" "Monaspace" "Monofur" "Monoid" "Mononoki" "MPlus" "NerdFontsSymbolsOnly" "Noto" "OpenDyslexic"
    "Overpass" "ProFont" "ProggyClean" "Recursive" "RobotoMono" "ShareTechMono" "SourceCodePro" "SpaceMono" "Terminus" "Tinos"
    "Ubuntu" "UbuntuMono" "UbuntuSans" "VictorMono" "ZedMono"
)

# Functions
print_color() {
    local color=$1
    local message=$2
    echo -e "${colors[$color]}$message${colors["reset"]}"
}

read_color() {
    local color=$1
    local message=$2
    read -p "$(echo -e "${colors[$color]}$message${colors["reset"]}")" response
    echo "$response"
}

is_sudo() {
    sudo -n true 2>/dev/null
}

check_cpu_brand() {
    #check if intel or amd
    if grep -qi "intel" /proc/cpuinfo; then
        echo "Intel"
    elif grep -qi "amd" /proc/cpuinfo; then
        echo "AMD"
    else
        echo "Unknown"
    fi
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

check_arch() {
    case $(uname -m) in
        "aarch64") echo "ARM64" ;;
        "x86_64") echo "x64" ;;
        *"arm"*) echo "ARM" ;;
        *) echo "Unknown" ;;
    esac
}

answer_default() {
    local answer=$1
    local default=$2
    [[ $answer == [Yy] || ($default == "y" && -z $answer) ]]
}

check_command() {
    local cmd=$1
    if command -v $cmd &>/dev/null; then
        print_color "green" "$cmd is already installed."
        return 0
    fi
    return 1
}

user_install_prompt() {
    local package=$1
    local package_install_function=$2
    # Would you like to install?
    answer=$(read_color "green" "Would you like to install $package? [Y/n]")
    if answer_default "$answer" "y"; then
        $package_install_function
    fi
}

# Function to install packages using dnf on Fedora 22+ and RHEL 8+
install_package() {
    local package=$1
    if command -v $package &>/dev/null; then
        print_color "green" "$package is already installed."
        return 0
    fi
    print_color "green" "Installing $package using $manager..."
    sudo dnf install -y $package
}

install_packages() {
    local packages=("$@")
    for package in "${packages[@]}"; do
        install_package "$package"
    done
}

install_with_nix() {
    install_nix
    for package in "$@"; do
        if nix-env -q | grep -q "^$package$"; then
            print_color "green" "$package is already installed via Nix."
        else
            print_color "green" "Installing $package using Nix..."
            nix-env -i "$package" || print_color "red" "Failed to install $package."
        fi
    done
}
install_with_flatpak() {
    install_flatpak
    for package in "$@"; do
        if flatpak list | grep -q "$package"; then
            print_color "green" "$package is already installed via Flatpak."
        else
            print_color "green" "Installing $package using Flatpak..."
            flatpak install flathub "$package" -y || print_color "red" "Failed to install $package."
        fi
    done
}

install_nerd_font() {
    local font=$1
    local fonts_dir="$HOME/.local/share/fonts"
    local font_dir="$fonts_dir/$font"
    local font_version="v3.3.0"
    local font_extension="zip"

    if [[ ! " ${nerd_font_list[@]} " =~ " ${font} " ]]; then
        print_color "red" "Error: Invalid font name. Please choose from the following:"
        print_color "yellow" "${nerd_font_list[*]}"
        return 1
    fi

    install_wget
    install_curl

    print_color "green" "Installing font $font..."
    local temp_dir=$(mktemp -d)
    mkdir -p "$font_dir"
    local font_url="https://github.com/ryanoasis/nerd-fonts/releases/download/$font_version/$font.$font_extension"

    if ! wget -q -P "$temp_dir" "$font_url"; then
        print_color "red" "Failed to download $font."
        return 1
    fi

    if [[ $font_extension == "zip" ]]; then
        unzip -q "$temp_dir/$font.$font_extension" -d "$temp_dir"
    elif [[ $font_extension == "tar.xz" ]]; then
        tar -xf "$temp_dir/$font.$font_extension" -C "$temp_dir"
    fi

    find "$temp_dir" -name '*.ttf' -exec cp {} "$font_dir" \;
    rm -rf "$temp_dir"
    print_color "green" "$font installed."
}

install_nerd_fonts() {
    local selected_fonts=()
    local available_fonts=("All" "${nerd_font_list[@]}" "Done")

    while true; do
        echo "Available Fonts:"
        select opt in "${available_fonts[@]}"; do
            if [[ "$opt" == "Done" ]]; then
                break 2
            elif [[ "$opt" == "All" ]]; then
                selected_fonts=("${nerd_font_list[@]}")
                break 2
            elif [[ -n "$opt" ]]; then
                if [[ ! " ${selected_fonts[@]} " =~ " ${opt} " ]]; then
                    selected_fonts+=("$opt")
                    print_color "green" "✔ Added: $opt"
                else
                    print_color "yellow" "⚠ $opt is already selected!"
                fi
                break
            else
                print_color "red" "❌ Invalid choice, please try again."
            fi
        done
    done

    print_color "blue" "Starting installation process..."
    for font in "${selected_fonts[@]}"; do
        install_nerd_font "$font" -d
    done
}

# Define available installation categories
install_fonts() {
    print_color "blue" "Installing fonts..."
    install_nerd_fonts
}

# Package Managers
install_nix() {
    if check_command nix-env; then
        return 0
    fi
    print_color "green" "Installing Nix..."
    sh <(curl -L https://nixos.org/nix/install) --daemon
    print_color "green" "Nix installed."
}
install_flatpak() {
    if check_command flatpak; then
        flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
        return 0
    fi
    print_color "green" "Installing Flatpak..."
    install_package flatpak
    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    print_color "green" "Flatpak installed."
}
install_package_managers() {
    print_color "blue" "Installing package managers..."
    user_install_prompt "Nix" install_nix
    user_install_prompt "Flatpak" install_flatpak
}

# Shells
install_fish() {
    install_package fish
}
install_zsh() {
    # Powerful shell with advanced features
    install_package zsh
}
install_shells() {
    print_color "blue" "Installing alternative shells..."
    user_install_prompt "Zsh" install_zsh
    user_install_prompt "fish" install_fish
}

# Install Programming Languages
install_go() {
    if check_command go; then
        return 0
    fi
    install_package golang
}
install_php() {
    install_package php

    if check_command composer; then
        return 0
    fi
    print_color "green" "Installing Composer..."
    # Download and verify the Composer installer
    EXPECTED_CHECKSUM="$(php -r 'copy("https://composer.github.io/installer.sig", "php://stdout");')"
    php -r "copy('https://getcomposer.org/installer', 'composer-setup.php');"
    ACTUAL_CHECKSUM="$(php -r "echo hash_file('sha384', 'composer-setup.php');")"

    if [ "$EXPECTED_CHECKSUM" != "$ACTUAL_CHECKSUM" ]; then
        print_color "red" "Composer installer checksum verification failed!"
        rm composer-setup.php
        return 1
    fi

    # Install Composer globally
    php composer-setup.php --install-dir=/usr/local/bin --filename=composer
    rm composer-setup.php
    print_color "green" "Composer installed successfully."
}
install_rust() {
    if check_command rustc; then
        return 0
    fi
    print_color "green" "Installing rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
    print_color "green" "rust installed."
}
install_dotnet() {
    if check_command dotnet; then
        return 0
    fi
    sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
    sudo dnf config-manager --add-repo https://packages.microsoft.com/fedora/$(rpm -E %fedora)/prod/
    install_package dotnet-sdk-9.0
}
install_python() {
    install_package python3
}
install_nodejs() {
    install_packages nodejs npm yarn
    if ! check_command bun; then
        print_color "green" "Installing Bun..."
        curl -fsSL https://bun.sh/install | bash
        print_color "green" "Bun installed successfully."
    fi
}
install_elixir() {
    install_package erlang
    install_package elixir
}
install_cpp() {
    if check_command g++; then
        return 0;
    fi
    print_color "green" "Installing C++ compiler..."
    sudo dnf install -y gcc-c++
    print_color "green" "C++ compiler installed successfully."
}
install_c() {
    install_package gcc
}
install_programming_languages() {
    print_color "blue" "Installing programming languages..."
    user_install_prompt "Go" install_go
    user_install_prompt "PHP" install_php
    user_install_prompt ".Net" install_dotnet
    user_install_prompt "Rust" install_rust
    user_install_prompt "Python" install_python
    user_install_prompt "NodeJS" install_nodejs
    user_install_prompt "Elixir" install_elixir
    user_install_prompt "C++" install_cpp
    user_install_prompt "C" install_c
}

install_intel_media_driver() {
    sudo dnf install libavcodec-freeworld --allowerasing -y
    sudo dnf install intel-media-driver --allowerasing -y
}

install_amd_media_driver() {
    sudo dnf install libavcodec-freeworld --allowerasing -y
    sudo dnf install mesa-va-drivers-freeworld --allowerasing -y
}
install_media_codecs() {
    print_color "blue" "Installing media codecs..."
    if check_cpu_brand | grep -q "Intel"; then
        install_intel_media_driver
    elif check_cpu_brand | grep -q "AMD"; then
        install_amd_media_driver
    fi
}
install_rpmfusion() {
    sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm -y
    sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y
}
install_system_improvements() {
    print_color "blue" "Installing system improvements..."
    install_media_codecs
}

# Desktop Environments
install_gnome_minimal() {
    # Install Minimal Gnome DE
    sudo dnf install @base-x gnome-shell gnome-terminal firefox nautilus -y
}
install_gnome() {
    # Install Full Gnome DE
    sudo dnf install @workstation-product-environment -y
}
install_kde() {
    # Install KDE Plasma DE
    sudo dnf install @kde-desktop-environment -y
}

install_desktop_environment() {
    print_color "blue" "Installing desktop environments..."
    user_install_prompt "Gnome" install_gnome
    user_install_prompt "KDE" install_kde
    user_install_prompt "Minimal Gnome" install_gnome_minimal
}

# Window Managers
install_i3() {
    # Install i3 Window Manager
    sudo dnf install i3 i3status dmenu i3lock xbacklight feh -y
}
install_sway() {
    # Install Sway Window Manager
    sudo dnf install sway waybar mako grim slurp kanshi swaylock -y
}
install_xmonad() {
    # Install Xmonad Window Manager
    sudo dnf install xmonad xmonad-contrib xmobar dmenu -y
}
install_bspwm() {
    # Install BSPWM Window Manager
    sudo dnf install bspwm sxhkd polybar -y
}

install_window_manager() {
    print_color "blue" "Installing window managers..."
    user_install_prompt "i3" install_i3
    user_install_prompt "Sway" install_sway
    user_install_prompt "Xmonad" install_xmonad
    user_install_prompt "BSPWM" install_bspwm
}

# Browsers
install_zenbrowser() {
    install_with_flatpak app.zen_browser.zen
}
install_firefox() {
    install_package firefox
}
install_bravebrowser() {
    install_with_flatpak com.brave.Browser
}
install_opera() {
    install_with_flatpak com.opera.Opera
}
install_opera_dev() {
    sudo rpm --import https://rpm.opera.com/rpmrepo.key
sudo tee /etc/yum.repos.d/opera.repo <<RPMREPO
[opera]
name=Opera packages
type=rpm-md
baseurl=https://rpm.opera.com/rpm
gpgcheck=1
gpgkey=https://rpm.opera.com/rpmrepo.key
enabled=1
RPMREPO

    sudo dnf install opera-developer
}
install_operagx() {
    flatpak install flathub com.usebottles.bottles
    print_color "yellow" "create a Bottle: by opening Bottles and creating a new 'bottle' (Wine environment). Then Download the Opera GX installer and use Bottles to install it in the new bottle. Launch Opera GX from within Bottles."
    open_url "https://www.opera.com/gx/gx-browser"
}
install_vivaldi() {
    install_with_flatpak com.vivaldi.Vivaldi
}
install_torbrowser() {
    install_with_flatpak com.github.micahflee.torbrowser-launcher
}
install_googlechrome() {
    install_with_flatpak com.google.Chrome
}
install_msedge() {
    install_with_flatpak com.microsoft.Edge
}
install_polypane() {
    install_with_flatpak com.polypane.Polypane
}
install_browsers() {
    print_color "blue" "Installing browsers..."
    install_flatpak
    user_install_prompt "Firefox" install_firefox
    user_install_prompt "Zen Browser" install_zenbrowser
    user_install_prompt "Opera" install_opera
    user_install_prompt "Opera Developer" install_opera_dev
    user_install_prompt "Opera GX" install_operagx
    user_install_prompt "Vivaldi" install_vivaldi
    user_install_prompt "Tor Browser" install_torbrowser
    user_install_prompt "Google Chrome" install_googlechrome
    user_install_prompt "MS Edge" install_msedge
    user_install_prompt "Polypane" install_polypane
}

# CLI Tools
install_wget() {
    install_package wget
}
install_curl() {
    install_package curl
}
install_git() {
    # Version control system
    install_package git
}
install_nvim() {
    # Modern Vim-based text editor
    install_package neovim
}
install_jq() {
    # Command-line JSON processor
    install_package jq
}
install_ripgrep() {
    # Faster grep alternative
    install_package ripgrep
}
install_cli_tools() {
    print_color "blue" "Installing CLI tools..."

    # Development Tools
    install_git             
    install_nvim            
    install_jq              
    install_ripgrep         
    install_package fd-find         # User-friendly find alternative
    install_package bat             # cat clone with syntax highlighting
    install_package exa             # Modern ls replacement
    install_package fzf             # Fuzzy finder for interactive filtering
    install_package tig             # Text-mode interface for Git
    install_package lazygit         # Terminal UI for Git commands
    install_package diff-so-fancy   # Better git diff output

    # Productivity Tools
    install_package tmux            # Terminal multiplexer
    install_package zsh             # Powerful shell with advanced features
    install_package fastfetch        # Display system information
    install_package taskwarrior     # Command-line task manager
    install_package ranger          # Terminal file manager
    install_package entr            # Run commands when files change
    install_package asciinema       # Record and share terminal sessions
    install_package glow            # Render markdown in the terminal

    # Miscellaneous Tools
    install_package cowsay          # Generate ASCII art of a cow with a message
    install_package figlet          # Create large text banners
    install_package lolcat          # Rainbow-colored terminal output
    install_package fortune         # Display random quotes or fortunes
    install_package cmatrix         # Simulate the Matrix movie's falling code
    install_package sl              # Steam Locomotive (fun command for typos)
}

install_kombrei() {
    install_packages cmake valac libgtk-3-dev libgee-0.8-dev libclutter-gtk-1.0-dev libclutter-1.0-dev libwebkit2gtk-4.0-dev libclutter-gst-3.0-dev
    git clone https://github.com/cheesecakeufo/komorebi.git
    cd komorebi
    mkdir build && cd build
    cmake .. && sudo make install && ./komorebi
}
install_kde_wallpaper_engine() {
    # Please add "RPM Fusion" repo first
    install_packages vulkan-headers plasma-workspace-devel kf5-plasma-devel gstreamer1-libav \
    lz4-devel mpv-libs-devel python3-websockets qt5-qtbase-private-devel libplasma-devel \
    qt5-qtx11extras-devel qt5-qtwebchannel-devel qt5-qtwebsockets-devel cmake
    
    # Download source
    git clone https://github.com/catsout/wallpaper-engine-kde-plugin.git
    cd wallpaper-engine-kde-plugin

    # Download submodule
    git submodule update --init --force --recursive

    # Configure, build and install
    # 'USE_PLASMAPKG=ON': using kpackagetool tool to install plugin
    cmake -B build -S . -GNinja -DUSE_PLASMAPKG=ON
    cmake --build build
    cmake --install build

    # Install package (ignore if USE_PLASMAPKG=OFF for system-wide installation)
    cmake --build build --target install_pkg
}
install_customization() {
    print_color "blue" "Installing customization tools..."
    # Check if KDE or Gnome
    install_kde_wallpaper_engine
}

# Communication
install_discord() {
    install_with_flatpak com.discordapp.Discord
}
install_telegram() {
    install_with_flatpak org.telegram.desktop
}
install_signal() {
    install_with_flatpak org.signal.Signal
}
install_slack() {
    install_with_flatpak com.slack.Slack
}
install_zoom() {
    install_with_flatpak us.zoom.Zoom
}
install_msteams() {
    install_with_flatpak com.microsoft.Teams
}
install_communication() {
    print_color "blue" "Installing communication apps..."
    user_install_prompt "Discord" install_discord
    user_install_prompt "Telegram" install_telegram
    user_install_prompt "Signal" install_signal
    user_install_prompt "Slack" install_slack
    user_install_prompt "Zoom" install_zoom
    user_install_prompt "Microsoft Teams" install_msteams
}

# Creative Tools
install_gimp() {
    install_with_flatpak org.gimp.GIMP
}
install_inkscape() {
    install_with_flatpak org.inkscape.Inkscape
}
install_krita() {
    install_with_flatpak org.kde.krita
}
install_davinciresolve() {
    install_with_flatpak com.blackmagicdesign.DaVinciResolve
}
install_blender() {
    install_with_flatpak org.blender.Blender
}
install_darktable() {
    install_with_flatpak org.darktable.Darktable
}
install_shotcut() {
    install_with_flatpak org.shotcut.Shotcut
}
install_freecad() {
    install_with_flatpak org.freecadweb.FreeCAD
}
install_openshot() {
    install_with_flatpak org.openshot.OpenShot
}
install_kdenlive() {
    install_with_flatpak org.kde.kdenlive
}
install_scribus() {
    install_with_flatpak net.scribus.Scribus
}
install_mypaint() {
    install_with_flatpak org.mypaint.MyPaint
}
install_synfig() {
    install_with_flatpak org.synfig.SynfigStudio
}
install_pencil2d() {
    install_with_flatpak org.pencil2d.Pencil2D
}
install_rawtherapee() {
    install_with_flatpak com.rawtherapee.RawTherapee
}
install_digikam() {
    install_with_flatpak org.kde.digikam
}
install_creative_tools() {
    print_color "blue" "Installing creative software..."
    user_install_prompt "GIMP" install_gimp
    user_install_prompt "Inkscape" install_inkscape
    user_install_prompt "Krita" install_krita
    user_install_prompt "DaVinci Resolve" install_davinciresolve
    user_install_prompt "Kdenlive" install_kdenlive
    user_install_prompt "Blender" install_blender
    user_install_prompt "Darktable" install_darktable
    user_install_prompt "Shotcut" install_shotcut
    user_install_prompt "FreeCAD" install_freecad
    user_install_prompt "OpenShot" install_openshot
    user_install_prompt "Scribus" install_scribus
    user_install_prompt "MyPaint" install_mypaint
    user_install_prompt "Synfig Studio" install_synfig
    user_install_prompt "Pencil2D" install_pencil2d
    user_install_prompt "RawTherapee" install_rawtherapee
    user_install_prompt "Digikam" install_digikam
}

# podman, docker, github desktop
install_podman() {
    install_packages podman podman-compose
}
install_docker() {
    if check_command -v docker; then
        return 0;
    fi
    print_color "green" "Installing Docker..."
    # Add Docker repository
    sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
    # Install Docker
    install_packages docker-ce docker-ce-cli containerd.io
    # Start and enable Docker service
    sudo systemctl start docker
    sudo systemctl enable docker
    # Add user to the Docker group
    sudo usermod -aG docker $USER
    print_color "green" "Docker installed successfully. Please log out and back in to apply group changes."
}
install_github_cli() {
    if command -v gh &>/dev/null; then
        print_color "green" "GitHub CLI is already installed."
        return 0
    fi
    print_color "green" "Installing GitHub CLI..."
    # Add GitHub CLI repository
    sudo dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo
    # Install GitHub CLI
    sudo dnf install -y gh
    print_color "green" "GitHub CLI installed successfully."
}
install_github_desktop() {
    if flatpak list | grep -q io.github.shiftey.Desktop; then
        print_color "green" "GitHub Desktop is already installed."
    else
        print_color "green" "Installing GitHub Desktop..."
        install_with_flatpak io.github.shiftey.Desktop
    fi
}
install_devbox() {
    if command -v devbox &>/dev/null; then
        print_color "green" "DevBox is already installed."
    else
        print_color "green" "Installing DevBox..."
        # Download and install DevBox
        curl -fsSL https://get.jetpack.io/devbox | bash
        print_color "green" "DevBox installed successfully."
    fi
}
install_dev_tools() {
    print_color "blue" "Installing development tools..."
    user_install_prompt "Podman" install_podman
    user_install_prompt "Docker" install_docker
    user_install_prompt "GitHub Cli" install_github_cli
    user_install_prompt "GitHub Desktop" install_github_desktop
    user_install_prompt "DevBox" install_devbox
}

install_vscode() {
    install_with_flatpak com.visualstudio.code
}
install_vscodium() {
    # Install vscodium
    sudo rpmkeys --import https://gitlab.com/paulcarroty/vscodium-deb-rpm-repo/-/raw/master/pub.gpg
    printf "[gitlab.com_paulcarroty_vscodium_repo]\nname=download.vscodium.com\nbaseurl=https://download.vscodium.com/rpms/\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=https://gitlab.com/paulcarroty/vscodium-deb-rpm-repo/-/raw/master/pub.gpg\nmetadata_expire=1h" | sudo tee -a /etc/yum.repos.d/vscodium.repo
    sudo dnf install codium -y
}
install_kate() {
    sudo dnf install kate -y
}
install_ides() {
    print_color "blue" "Installing IDEs..."
    install_vscode
    install_vscodium
    install_kate
}

install_video_players() {
    print_color "blue" "Installing video players..."
    install_with_flatpak org.videolan.VLC
}

# Audio Tools
install_audacity() {
    install_with_flatpak org.audacityteam.Audacity
}
install_ardour() {
    install_with_flatpak org.ardour.Ardour
}
install_lmms() {
    install_with_flatpak io.lmms.LMMS
}
install_hydrogen() {
    install_with_flatpak org.hydrogenmusic.Hydrogen
}
install_musescore() {
    install_with_flatpak org.musescore.MuseScore
}
install_audio_tools() {
    print_color "blue" "Installing audio production tools..."
    user_install_prompt "Audacity" install_audacity
    user_install_prompt "Ardour" install_ardour
    user_install_prompt "LMMS" install_lmms
    user_install_prompt "Hydrogen" install_hydrogen
    user_install_prompt "MuseScore" install_musescore
}

install_productivity() {
    print_color "blue" "Installing productivity applications..."

    # Office Suites
    install_with_flatpak org.onlyoffice.desktopeditors  # OnlyOffice
    install_with_flatpak org.libreoffice.LibreOffice    # LibreOffice

    # Note-Taking and Documentation
    install_with_flatpak org.gnome.Notes                # GNOME Notes
    install_with_flatpak org.zim_project.Zim            # Zim Desktop Wiki
    install_with_flatpak md.obsidian.Obsidian           # Obsidian (markdown-based notes)

    # Task Management
    install_with_flatpak org.gnome.Todo                 # GNOME To Do
    install_with_flatpak com.github.alainm23.planner    # Planner (task manager)
    install_with_flatpak com.github.phase1geo.minder    # Minder (mind mapping tool)

    # PDF and Document Tools
    install_with_flatpak org.kde.okular                 # Okular (PDF viewer)
    install_with_flatpak net.xmind.XMind                # XMind (mind mapping tool)
    install_with_flatpak org.kde.skanpage               # Skanpage (document scanner)
}

install_virtualization_containers() {
    print_color "blue" "Installing virtualization software..."
    # Virtualization Tools
    install_package virtualbox                         # VirtualBox
    install_package virt-manager                       # Virt-Manager (KVM/QEMU GUI)
    install_package qemu-kvm                           # QEMU/KVM (hypervisor)
    install_package libvirt                            # Libvirt (virtualization API)

    # Containerization Tools
    install_podman                             # Podman (container runtime)
    install_docker                             # Docker (container runtime)
    install_with_dnf buildah                            # Buildah (container image builder)
    install_with_dnf skopeo                             # Skopeo (container image management)

    # Container Orchestration
    install_with_dnf kubernetes-client                  # Kubernetes CLI (kubectl)
    install_with_dnf minikube                           # Minikube (local Kubernetes cluster)

    # Miscellaneous
    install_with_dnf vagrant                            # Vagrant (virtual machine management)
    install_with_dnf terraform                          # Terraform (infrastructure as code)
}

install_networking() {
    print_color "blue" "Installing networking tools..."

    # Network Analysis and Debugging
    install_package nmap            # Network exploration and security auditing
    install_package nmap-ncat       # Netcat utility for networking tasks
    install_package httpie          # User-friendly HTTP client
    install_package wget2           # Next-generation wget with improved performance
    install_package curl            # Command-line tool for transferring data
    install_package openssh         # SSH client and server
    install_package mtr             # Network diagnostic tool (combines ping and traceroute)
    install_package iperf3          # Network performance testing tool
    install_package tcpdump         # Packet analyzer for network troubleshooting
    install_package wireshark       # Network protocol analyzer (GUI and CLI)
    install_package net-tools       # Basic networking tools (ifconfig, netstat, etc.)
    install_package iputils         # Utilities like ping and traceroute
    install_package bind-utils      # DNS troubleshooting tools (dig, nslookup)
}
install_unity() {
    install_with_flatpak com.unity.UnityHub
}
install_renpy() {
    local version="8.3.4"
    local url="https://www.renpy.org/dl/$version/renpy-$version-sdk.tar.bz2"
    local install_dir="$HOME/.local/share/renpy"
    local install_path="$install_dir/$version"
    local desktop_file="$HOME/.local/share/applications/renpy.desktop"

    # Check if Ren'Py is already installed
    if [ -d "$install_path" ]; then
        print_color "yellow" "Ren'Py $version is already installed at $install_path."

        # Ask user if they want to reinstall
        read -p "Do you want to reinstall? (y/N): " choice
        choice=${choice,,} # Convert to lowercase

        if [[ "$choice" != "y" && "$choice" != "yes" ]]; then
            print_color "green" "Installation aborted. Ren'Py is already installed."
            return 0
        fi

        print_color "red" "Reinstalling Ren'Py $version..."
        rm -rf "$install_path"
        rm -f $desktop_file
    fi

    # Create install directory if it doesn't exist
    mkdir -p "$install_dir"

    # Download Ren'Py
    print_color "purple" "Downloading Ren'Py version $version..."
    curl -L -o "$install_dir/renpy-$version-sdk.tar.bz2" "$url"

    # Extract the archive
    print_color "purple" "Extracting Ren'Py..."
    tar -xvf "$install_dir/renpy-$version-sdk.tar.bz2" -C "$install_dir"

    # Cleanup downloaded archive
    rm "$install_dir/renpy-$version-sdk.tar.bz2"

    # Rename extracted folder to version number
    mv "$install_dir/renpy-$version-sdk" $install_path

    # Add Ren'Py to PATH if not already added
    if ! grep -q "$install_path" ~/.profile; then
        echo 'export PATH="'"$install_path"':$PATH"' >> ~/.profile
        source ~/.profile
    fi

    # Create a .desktop file to add Ren'Py to the application menu
    print_color "blue" "Creating Ren'Py desktop entry..."
    cat <<EOF > "$desktop_file"
[Desktop Entry]
Name=Ren'Py
Description=Visual Novel Game Engine
Comment=Multi-platform visual novel game engine
Exec=$install_path/renpy.sh
Icon=$install_path/launcher/icon.icns
Terminal=false
Type=Application
Categories=Development;
StartupNotify=true
EOF

    # Ensure the .desktop file is executable
    chmod +x "$desktop_file"

    # Refresh application database
    update-desktop-database ~/.local/share/applications

    # Final success message
    print_color "green" "Ren'Py $version installed successfully!"
    print_color "green" "You can now find it in the application menu or run it with:"
    print_color "green" "cd $install_path && ./renpy.sh"
}

install_unreal_engine() {
    install_with_flatpak com.epicgames.UnrealEngine
}
install_godot() {
    # Godot (Standard Version)
    install_with_flatpak org.godotengine.Godot
    # Godot Mono (C# Support)
    install_with_flatpak org.godotengine.GodotSharp
}

install_steam() {
    install_package steam-devices
    install_with_flatpak com.valvesoftware.Steam
}
install_lutris() {
    install_with_flatpak net.lutris.Lutris
}
install_gaming_game_development() {
    print_color "blue" "Installing game development software..."
    # Gaming Apps
    user_install_prompt "Steam" install_steam
    user_install_prompt "Lutris" install_lutris
    # Game Development Tools
    user_install_prompt "Godot" install_godot
    user_install_prompt "Ren'Py" install_renpy
    user_install_prompt "Unity Hub" install_unity
    user_install_prompt "Unreal Engine" install_unreal_engine
}

install_security_privacy() {
    print_color "blue" "Installing security and privacy tools..."
    # Password Management
    install_package keepassxc          # Password manager
    install_package bitwarden          # Open-source password manager (CLI)
    # Encryption Tools
    install_package gnupg              # GNU Privacy Guard for encryption
    install_package veracrypt          # Disk encryption software
}

install_streaming_recording() {
    print_color "blue" "Installing streaming and recording software..."
    install_with_flatpak com.obsproject.Studio
}

install_tmux() {
    # Terminal multiplexer
    install_package tmux
}
install_fastfetch() {
    # Display system information
    install_package fastfetch
}
install_tig() {
    # Text-mode interface for Git
    install_package tig
}
install_lazygit() {
    # Terminal UI for Git commands
    install_package lazygit
}
install_diff_so_fancy() {
    # Better git diff output
    install_package diff-so-fancy
}
install_htop() {
    # Interactive process viewer
    install_package htop
}
install_btop() {
    # Modern system monitor
    install_package btop
}
install_glances() {
    # Cross-platform system monitoring
    install_package glances
}
install_ncdu() {
    # Disk usage analyzer
    install_package ncdu
}
install_lsof() {
    # List open files and processes
    install_package lsof
}
install_fd_find(){
    # User-friendly find alternative
    install_package fd-find
}
install_bat() {
    # cat clone with syntax highlighting
    install_package bat
}
install_exa() {
    # Modern ls replacement
    install_package exa
}
install_fzf() {
    # Fuzzy finder for interactive filtering
    install_package fzf
}
install_tree() {
    # Display directory structures as a tree
    install_package tree
}
install_unzip() {
    # Extract .zip files
    install_package unzip
}
install_rsync() {
    # Fast and versatile file copying
    install_package rsync
}
install_utilities() {
    print_color "blue" "Installing system utilities..."
    # File and Text Manipulation
    install_ripgrep
    install_fd-find
    install_bat
    install_exa
    install_fzf
    install_tree
    install_unzip
    install_rsync

    # System Monitoring and Debugging
    install_htop
    install_btop
    install_glances
    install_ncdu
    install_lsof

    # Miscellaneous Utilities
    install_tmux
    install_fastfetch
    install_tig
    install_lazygit
    install_diff-so-fancy
}

# Themes
install_oh_my_zsh() {
    install_zsh
    install_git
    if [ -d "$HOME/.oh-my-zsh" ]; then
        print_color "green" "Oh My Zsh is already installed."
        return 0
    fi
    print_color "green" "Installing Oh My Zsh..."
    sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
    print_color "green" "Oh My Zsh installed."
}
install_powerlevel10k() {
    install_git
    install_zsh
    git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ~/powerlevel10k
    echo 'source ~/powerlevel10k/powerlevel10k.zsh-theme' >>~/.zshrc
}
install_nvchad() {
    install_nvim
    git clone https://github.com/NvChad/starter ~/.config/nvim && nvim
}
install_customization_theming() {
    print_color "blue" "Installing Customization Tools..."
    user_install_prompt "Oh My ZSH" install_oh_my_zsh
    user_install_prompt "powerlevel10k" install_powerlevel10k
    user_install_prompt "NVChad" install_nvchad
}

# Install available software categories
install_category() {
    case "$1" in
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
        *) print_color "red" "Invalid category: $1" ;;
    esac
}
update() {
    local free_pkg="rpmfusion-free-release"
    local nonfree_pkg="rpmfusion-nonfree-release"

    # Check if RPM Fusion is already installed
    if ! rpm -q "$free_pkg" &>/dev/null && rpm -q "$nonfree_pkg" &>/dev/null; then
        print_color "green" "Installing RPM Fusion free."
        yes | install_packages \
        https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
    fi

    if ! rpm -q "$nonfree_pkg" &>/dev/null; then
        print_color "green" "Installing RPM Fusion non-free."
        yes | install_packages \
        https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
    fi

    install_system_improvements
    
    print_color "purple" "Running System Update..."
    # Update system packages
    sudo dnf update -y
}

start() {
    local selected_categories=()
    local input=""
    
    echo "Available Categories:"
    for i in "${!categories[@]}"; do
        echo "$((i + 1)). ${categories[i]}"
    done
    echo "$(( ${#categories[@]} + 1 )). All"

    while true; do
        read -p "Enter numbers separated by commas (e.g., 1,3,5) or 'All' (Press Enter to continue): " input

        # Trim spaces and convert to lowercase
        input=$(echo "$input" | tr -d '[:space:]' | tr '[:upper:]' '[:lower:]')

        if [[ "$input" == "all" ]]; then
            selected_categories=("${categories[@]}")
            print_color "blue" "Installing all available software..."
            break
        elif [[ -z "$input" ]]; then
            if [[ ${#selected_categories[@]} -eq 0 ]]; then
                print_color "yellow" "⚠ No categories selected. Exiting..."
                return 0
            fi
            break
        fi

        # Convert user input into an array
        IFS=',' read -r -a selections <<< "$input"

        # Validate user input
        for choice in "${selections[@]}"; do
            if [[ "$choice" =~ ^[0-9]+$ ]] && (( choice > 0 && choice <= ${#categories[@]} )); then
                category="${categories[choice-1]}"
                if [[ ! " ${selected_categories[@]} " =~ " ${category} " ]]; then
                    selected_categories+=("$category")
                    print_color "green" "✔ Added: $category"
                else
                    print_color "yellow" "⚠ $category is already selected!"
                fi
            else
                print_color "red" "❌ Invalid selection: $choice"
            fi
        done

        # Confirm selection and allow re-selection
        read -p "Proceed with selected categories? (y/N): " confirm
        confirm=${confirm,,} # Convert to lowercase
        if [[ "$confirm" == "y" || "$confirm" == "yes" ]]; then
            break
        else
            selected_categories=()
        fi
    done

    print_color "blue" "Starting installation process..."
    update

    for category in "${selected_categories[@]}"; do
        install_category "$category"
    done

    print_color "green" "✅ Installation completed for: ${selected_categories[*]}"
}


start
