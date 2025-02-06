#!/bin/sh
source ./functions.sh

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

# Install shell
read -p "Do you want to install an additional shell? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install the fish shell? [Y/n] " answer
    if answer_default_y "$answer"; then
        echo "Installing fish..."
        brew install fish
    fi
fi

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

    read -p "Do you want to install protoc? [Y/n] " answer
    if answer_default_y "$answer"; then
        brew install protoc protoc-gen-go
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

# Browsers
read -p "Do you want to install a browser? [Y/n] " answer
if answer_default_y "$answer"; then
    read -p "Do you want to install Brave browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        pkg_man_install brave-browser
    fi

    read -p "Do you want to install LibreWolf browser? [Y/n] " answer
    if answer_default_y "$answer"; then
        pkg_man_install librewolf --no-quarantine
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
