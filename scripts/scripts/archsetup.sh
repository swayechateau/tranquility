#!/bin/bash
source ./functions.sh
# install shells
install_with_pacman zsh fish neovim git gnome xorg xorg-server

install_yay

sudo systemctl start gdm
sudo systemctl enable gdm4r

sudo pacman -S gnome-tweaks