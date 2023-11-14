use crate::utils::{contents::print_installer_info, core::*};
use colored::Colorize;
use installer::*;
use std::io;

mod installer;
mod utils;

fn main() -> io::Result<()> {
    let paths: Paths = Paths::build(); // All paths needed

    // Installer
    print_installer_info();
    installation_prompt()?;

    match check_config_dir(&paths.config) {
        Ok(()) => success!("==> .config directory found or new one created"),
        Err(error) => {
            error!("Could not create ~/.config directory. Exiting...", error);
            return Err(error);
        }
    }

    match clone_repo(&paths.repo) {
        Ok(DownloadStatus::Success) => {
            success!("==> Successfully cloned Github repo into ~/Downloads")
        }
        Ok(DownloadStatus::Existing) => {
            success!("==> Repo has already been cloned into ~/Downloads")
        }
        Err(error) => {
            error!(
                "Could not clone Github repo into ~/Downloads. Exiting...",
                error
            );
            return Err(error);
        }
    }

    match download_wallpaper(&paths.downloads) {
        Ok(DownloadStatus::Success) | Ok(DownloadStatus::Existing) => {
            success!("==> Wallpaper is located at ~/Downloads/flowers.png");
            match set_wallpaper(&paths.downloads, &paths.documents) {
                Ok(()) => success!(
                    "==> Wallpaper has been set to location ~/Documents/wallpapers successfully"
                ),
                Err(error) => error!("Could not set wallpaper", error),
            }
        }
        Err(error) => error!(
            "Could not download wallpaper. Ensure `wget` is installed on your system!",
            error
        ),
    }

    match cleanup_repo(&paths.home, &paths.repo) {
        Ok(()) => success!("==> Cleanup was successful"),
        Err(error) => error!(
            "There was an error while cleaning up ~/Downloads/arch-everforest",
            error
        ),
    }

    match create_backup(&paths.config, &paths.documents) {
        Ok(BackupStatus::Created) => {
            success!("==> Successfully created backup at ~/Documents/backup")
        }
        Ok(BackupStatus::Existing) => {
            success!("==> Using already existing backup at ~/Documents/config_backup")
        }
        Ok(BackupStatus::NoBackup) => success!("==> Continuing installation without backup"),
        Err(error) => {
            error!(
                "Could not create a backup of your ~/.config directory:",
                error
            );
            return Err(error);
        }
    }

    match copy_config_dirs_recursively(&paths.repo, &paths.config) {
        Ok(()) => {
            success!("==> Successfully copied config files to ~/.config!");
        }
        Err(error) => {
            error!("Could not copy files to ~/.config. Exiting...", error);
            return Err(error);
        }
    }

    match change_kb_layout() {
        Ok(KBLayoutStatus::Changed(layout)) => {
            println!(
                "{} -> {}",
                "==> Successfully changed keyboard layout to".green(),
                layout.green().bold()
            )
        }
        Ok(KBLayoutStatus::Default) => {
            success!("==> Using default keyboard layout: us")
        }
        Err(error) => error!("Changing keyboard layout failed", error),
    }

    match install_cli_utilities(&paths.home, &paths.config) {
        Ok(DownloadStatus::Success) => success!("==> Successfully installed CLI utilities"),
        Ok(DownloadStatus::Existing) => {}
        Err(error) => error!("Installing CLI utilities failed", error),
    }

    match after_install(&paths.repo) {
        Ok(()) => pause().unwrap(),
        Err(error) => error!("", error),
    }

    Ok(())
}
