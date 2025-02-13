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
declare -A nerd_font_list=(
    ["3270"]="3270"
    ["0xproto"]="0xProto"
    ["agave"]="Agave"
    ["anonymouspro"]="AnonymousPro"
    ["arimon"]="Arimo"
    ["aurulentsansm"]="AurulentSansMono"
    ["bigblueterm"]="BigBlueTerminal"
    ["bitstromwera"]="BitstreamVeraSansMono"
    ["cascadiacode"]="CascadiaCode"
    ["cascadiamono"]="CascadiaMono"
    ["codenewroman"]="CodeNewRoman"
    ["comicshannsmono"]="ComicShannsMono"
    ["commitmono"]="CommitMono"
    ["cousine"]="Cousine"
    ["d2coding"]="D2Coding"
    ["daddytimemono"]="DaddyTimeMono"
    ["dejavusansmono"]="DejaVuSansMono"
    ["departuremono"]="DepartureMono"
    ["droidsansmono"]="DroidSansMono"
    ["envycoder"]="EnvyCodeR"
    ["fantasquesansmono"]="FantasqueSansMono"
    ["firacode"]="FiraCode"
    ["firamono"]="FiraMono"
    ["fontpatcher"]="FontPatcher"
    ["geistmono"]="GeistMono"
    ["go-mono"]="Go-Mono"
    ["gohu"]="Gohu"
    ["hack"]="Hack"
    ["hasklig"]="Hasklig"
    ["heavydata"]="HeavyData"
    ["hermit"]="Hermit"
    ["ia-writer"]="iA-Writer"
    ["ibmplexmono"]="IBMPlexMono"
    ["inconsolata"]="Inconsolata"
    ["inconsolatago"]="InconsolataGo"
    ["inconsolatalgc"]="InconsolataLGC"
    ["intelonemono"]="IntelOneMono"
    ["iosevka"]="Iosevka"
    ["iosevkaterm"]="IosevkaTerm"
    ["iosevkatermslab"]="IosevkaTermSlab"
    ["jetbrainsmono"]="JetBrainsMono"
    ["lekton"]="Lekton"
    ["liberationmono"]="LiberationMono"
    ["lilex"]="Lilex"
    ["martianmono"]="MartianMono"
    ["meslo"]="Meslo"
    ["monaspace"]="Monaspace"
    ["monofur"]="Monofur"
    ["monoid"]="Monoid"
    ["mononoki"]="Mononoki"
    ["mplus"]="MPlus"
    ["nerdfontssymbolsonly"]="NerdFontsSymbolsOnly"
    ["noto"]="Noto"
    ["opendyslexic"]="OpenDyslexic"
    ["overpass"]="Overpass"
    ["profont"]="ProFont"
    ["proggyclean"]="ProggyClean"
    ["recursive"]="Recursive"
    ["robotomono"]="RobotoMono"
    ["sharetechmono"]="ShareTechMono"
    ["sourcecodepro"]="SourceCodePro"
    ["spacemono"]="SpaceMono"
    ["terminus"]="Terminus"
    ["tinos"]="Tinos"
    ["ubuntu"]="Ubuntu"
    ["ubuntumono"]="UbuntuMono"
    ["ubuntusans"]="UbuntuSans"
    ["victormono"]="VictorMono"
    ["zedmono"]="ZedMono"
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
    if [[ "$arch" == "x86_64" ]]; then
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

show_nerd_fonts(){
    # show the list of available fonts
    echo -e "${textyellow}Available Nerd Fonts:${textreset}"
    for font in "${!nerd_font_list[@]}"; do
        echo -e "${textyellow}$font${textreset}"
    done
    return 0
}

check_nerd_font() {
    local $font = $1

    # convert font name to lowercase, then check if it exists in the font_names array
    if [[ -z ${nerd_font_list[${font,,}]} ]]; then
        echo -e "${textred}Error: Font not found. Please choose from the following:${textreset}"
        show_nerd_fonts
        return 1
    fi

    return 0
}

add_nerd_font() {
    local $font = $1

    # convert font name to lowercase, then check if it exists in the font_names array
    if [[ -z ${font_names[${font,,}]} ]]; then
        echo -e "${textred}Error: Invalid font name. Please choose from the following:${textreset}"
        echo -e "${textyellow}${!font_names[*]}${textreset}"
        return 1
    fi

    # Ask if the user wants to install the font as zip or tar.xz
    echo -e "${textgreen}Do you want to install the font as a zip or tar.xz file?${textreset}"
    echo -e "${textyellow}1) zip${textreset}"
    echo -e "${textyellow}2) tar.xz${textreset}"
    read -p "" answer
    if [[ $answer == "1" ]]; then
        font_extension="zip"
    elif [[ $answer == "2" ]]; then
        font_extension="tar.xz"
    else
        font_extension="tar.xz"
    fi

    font_dir="$HOME/.local/share/fonts"
    font_version="v3.3.0"
    font_name="3270"
    font_url="https://github.com/ryanoasis/nerd-fonts/releases/download/$font_version/$font_slug.$font_extension"
    
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

# Tests
if is_sudo ; then
    echo "You are root"
else
    echo "You are not root"
fi
check_arch
show_nerd_fonts
