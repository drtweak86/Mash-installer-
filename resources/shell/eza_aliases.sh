#!/bin/bash
# Goblin's Mega eza Aliases
# Save as ~/.eza_aliases and source in your .zshrc/.bashrc

# Basic ls replacements
alias ls='eza --icons --group-directories-first'
alias ll='eza -lah --icons --group-directories-first'
alias lt='eza --tree --level=2 --icons'
alias lsg='eza -lah --git --icons'
alias lss='eza -lah --sort=size --icons'  # Sort by size
alias lsd='eza -lah --sort=modified --icons'  # Sort by date
alias lsf='eza -lah --filter-by-type=file --icons'  # Files only
alias lsdirs='eza -lah --filter-by-type=directory --icons'  # Dirs only

# Git aliases
alias gs='git status'
alias ga='git add .'
alias gc='git commit -m'
alias gcm='git checkout main'
alias gl='git log --oneline --graph --decorate'
alias gd='git diff'
alias gdc='git diff --cached'
alias gst='lazygit'

# System aliases
alias update='sudo pacman -Syu && yay -Syu'
alias clean='sudo pacman -Rns $(pacman -Qdtq)'
alias myip='curl ifconfig.me'
alias ports='ss -tulnp'
alias psg='ps aux | grep -i'
alias killport='sudo kill -9 $(lsof -i :$1 -t) 2>/dev/null'

# File ops
alias cp='cp -iv'
alias mv='mv -iv'
alias rm='trash-put'
alias mkcd='mkdir -p "$1" && cd "$1"'
alias edit='helix'

# Fun
alias weather='curl wttr.in'
alias cheat='curl cht.sh/:$1'
alias goblin='cmatrix -a'
alias motd='fortune | cowsay'
