# Auto-Ricer

<p align="center"><img src="hyprforest_logo.png" /></p>

<p align="center"><em>A small <strong>auto-ricer</strong> for Hyprland on Arch Linux!</em></p>

This installer will take all the configuration files from this repository _(https://github.com/3rfaan/dotfiles)_ and copies them into your 📁 **~/.config** directory.

**⚠️ NOTE: You have to install all the necessary programs from this [Guide](https://github.com/3rfaan/dotfiles/blob/main/README.md), otherwise the ricing won't have an effect on your system!**

## Preview

<img src="https://raw.githubusercontent.com/3rfaan/dotfiles/refs/heads/main/preview_1.png" />

## Installation

### [Crates.io](https://crates.io/crates/autoricer)

```
$ cargo install autoricer
```

You can then run the following command:

```
$ autoricer
```

### [AUR](https://aur.archlinux.org/packages/autoricer-bin)

```
$ yay -S autoricer-bin
```

You can then run the following command:

```
$ autoricer
```

### Build from Source

Execute the following commands in your terminal:

```
$ git clone https://github.com/3rfaan/autoricer.git $HOME/Downloads/autoricer && cd $HOME/Downloads/autoricer
$ cargo run
```

_Note: Rust has to be installed on the system to build from source!_

## Backup

A backup of all your old configuration files in 📁 **~/.config** will be created in 📁 **~/Documents/config_backup** before deleting anything.

## Change keyboard layout

When prompted you can change the keyboard layout directly in the installer.

## Nvidia Support

When prompted you can enable support for Nvidia in Hyprland. The installer will then put the appropriate environment variables inside Hyprland config file.

## After Installation

After the installation process you probably have to restart Hyprland using <kbd>super</kbd> + <kbd>shift</kbd> + <kbd>e</kbd>.

The wallpaper can be changed inside 📁 **~/.config/hypr/hyprpaper.conf**
