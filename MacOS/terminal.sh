#!/bin/sh

brew tap homebrew/cask-fonts
brew install --cask font-<FONT NAME>-nerd-font
# jetbrains-mono

# Set up vim
echo 'alias vi=nvim' >> $HOME/.bashrc   # if using bash
echo 'alias vi=nvim' >> $HOME/.zshrc   # if using zsh
echo 'alias vi=nvim' >> $HOME/.config/fish/config.fish   # if using fish

git clone https://github.com/NvChad/NvChad $HOME/.config/nvim --depth 1

# Windows
# \AppData\Local\nvim --depth 1

# Set up tmux
brew install tmux

git clone https://github.com/tmux-plugins/tpm  $HOME/.tmux/plugins/tpm
touch $HOME/.tmux/tmux.conf
cat << EOF > $HOME/.config/tmux/tmux.conf
# List of plugins
set -g @plugin 'tmux-plugins/tpm'
set -g @plugin 'tmux-plugins/tmux-sensible'

# Other examples:
# set -g @plugin 'github_username/plugin_name'
# set -g @plugin 'github_username/plugin_name#branch'
# set -g @plugin 'git@github.com:user/plugin'
# set -g @plugin 'git@bitbucket.com:user/plugin'

# Initialize TMUX plugin manager (keep this line at the very bottom of tmux.conf)
run '~/.tmux/plugins/tpm/tpm'
EOF

tmux
tmux source ~/.config/tmux/tmux.conf