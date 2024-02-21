#/bin/bash

echo "Installing apps for current system"
echo "user pretham"
cd

#list of apps

sudo apt update
sudo apt upgrade

sudo apt install kitty
sudo apt install neofetch
sudo apt install bashtop
sudo apt install git

git clone "https://github.com/PrethamMuthappa/Dotfiles.git"

sudo apt install nodejs
sudo apt install python3
sudo apt install pip
sudo apt install gnome-tweaks

#for helix

sudo add-apt-repository ppa:maveonair/helix-editor
sudo apt update
sudo apt install helix

cd

sudo apt install npm
sudo apt install ubuntu-cleaner
sudo apt install plocate
sudo apt install mysql-server mysql-client
sudo apt-get install simplescreenrecorder
sudo apt install synaptic

#for rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#lsp for helix
rustup component add rust-analyzer
npm i -g bash-language-server
npm i -g vscode-langservers-extracted
npm install -g typescript typescript-language-server
npm i -g vscode-langservers-extracted
pip install -U 'python-lsp-server[all]'
npm i -g @tailwindcss/language-server





