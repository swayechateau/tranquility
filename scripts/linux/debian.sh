#!/bin/bash

# Function to install PHP
install_php() {
    echo "Installing PHP..."
    sudo apt update && sudo apt install -y php
    echo "PHP installed."
}

# Function to install Composer
install_composer() {
    echo "Installing Composer..."
    wget -q -O composer-setup.php https://getcomposer.org/installer
    php composer-setup.php --install-dir=/usr/local/bin --filename=composer
    rm composer-setup.php
    echo "Composer installed."
}

# Function to install Docker
install_docker() {
    echo "Installing Docker..."
    sudo apt update && sudo apt install -y \
        ca-certificates \
        curl \
        gnupg \
        lsb-release
    curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    echo \  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \$(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    sudo apt update && sudo apt install -y docker-ce docker-ce-cli containerd.io
    echo "Docker installed."
}

# Function to install Wget
install_wget() {
    echo "Installing Wget..."
    sudo apt update && sudo apt install -y wget
    echo "Wget installed."
}

# Function to install Git
install_git() {
    echo "Installing Git..."
    sudo apt update && sudo apt install -y git
    echo "Git installed."
}

# Function to install Go
install_go() {
    echo "Installing Go..."
    wget https://go.dev/dl/go1.20.8.linux-amd64.tar.gz -O go.tar.gz
    sudo tar -C /usr/local -xzf go.tar.gz
    echo "export PATH=\$PATH:/usr/local/go/bin" >> ~/.bashrc
    source ~/.bashrc
    rm go.tar.gz
    echo "Go installed."
}

# Main menu
main_menu() {
    echo "Select an option:"
    echo "1. Install All Server"
    echo "2. Install All Desktop"
    echo "3. Custom Install"
    echo "4. Exit"
    read -rp "Enter your choice [1-3]: " choice

    case $choice in
        1)
            install_php
            install_composer
            install_docker
            install_wget
            install_git
            install_go
            ;;
        2)
            install_php
            install_composer
            install_docker
            install_wget
            install_git
            install_go
            ;;
        3)
            echo "Select components to install:"
            read -p "Install PHP? (y/n): " php_choice
            if [[ $php_choice == "y" ]]; then
                install_php
            fi

            read -p "Install Composer? (y/n): " composer_choice
            if [[ $composer_choice == "y" ]]; then
                install_composer
            fi

            read -p "Install Docker? (y/n): " docker_choice
            if [[ $docker_choice == "y" ]]; then
                install_docker
            fi

            read -p "Install Wget? (y/n): " wget_choice
            if [[ $wget_choice == "y" ]]; then
                install_wget
            fi

            read -p "Install Git? (y/n): " git_choice
            if [[ $git_choice == "y" ]]; then
                install_git
            fi

            read -p "Install Go? (y/n): " go_choice
            if [[ $go_choice == "y" ]]; then
                install_go
            fi
            ;;
        4)
            echo "Exiting."
            exit 0
            ;;
        *)
            echo "Invalid choice. Exiting."
            exit 1
            ;;
    esac
}

# Run the main menu
main_menu
