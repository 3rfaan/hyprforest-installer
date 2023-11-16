# Hyprforest

<p align="center"><em>A small <strong>Everforest Theme Installer</strong> for Hyprland on Arch Linux!</em></p>

This installer will take all the configuration files from this repository _(https://github.com/3rfaan/arch-everforest)_ and copies them into your  📁 **~/.config** directory.

## Installation

### Cargo

```
$ cargo install hyprforest-installer
```

You can then run the following command: 

```
$ hyprforest-installer
```

### AUR

```
$ yay -S hyprforest-installer-bin
```

You can then run the following command: 

```
$ hyprforest-installer
```

### Build from Source

Execute the following commands in your terminal:

```
$ git clone https://github.com/3rfaan/hyprforest-installer.git $HOME/Downloads/hyprforest-installer && cd $HOME/Downloads/hyprforest-installer
$ cargo run
```

_Note: Rust has to be installed on the system!_

## Backup

A backup of all your old configuration files in 📁 **~/.config** will be created in 📁 **~/Documents/config_backup** before deleting anything.

## Change keyboard layout

When prompted you can change the keyboard layout directly in the installer.

## Nvidia Support

When prompted you can enable support for Nvidia in Hyprland. The installer will then put the appropriate environment variables inside Hyprland config file.

## After Installation

After the installation process you probably have to restart Hyprland using <kbd>super</kbd> + <kbd>shift</kbd> + <kbd>e</kbd>.

The wallpaper can be changed inside 📁 **~/.config/hypr/hyprpaper.conf**
