# Maintainer: Arfan Zubi
# Maintainer: <zubi.arfan@gmail.com
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=hyprforest-installer-bin
pkgver=0.1.2
pkgrel=1
pkgdesc="Everforest Theme Installer for Hyprland on Arch Linux"
url="https://github.com/3rfaan/hyprforest-installer"
license=("GPL-3.0-or-later")
arch=("x86_64")
provides=("hyprforest-installer")
conflicts=("hyprforest-installer")
optdepends=("hyprland" "kitty" "neovim" "waybar" "wofi" "zsh")
source=("https://github.com/3rfaan/hyprforest-installer/releases/download/v$pkgver/hyprforest-installer-$pkgver-x86_64.tar.gz")
sha256sums=("b160099955d558ae5193e657714e0cfd12348ae8c8bad8f29f09fcffcc955bcc")

package() {
    install -Dm755 hyprforest-installer -t "$pkgdir/usr/bin"
}
