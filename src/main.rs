use crate::utils::{contents::print_installer_info, functions::pause, types::*};
use colored::Colorize;
use installer::*;
use std::io;

mod installer;
mod utils;

fn main() -> io::Result<()> {
    let paths: Paths = Paths::build(); // All paths needed

    // Installer
    print_installer_info();

    match installation_prompt() {
        Ok(Installation::Proceed) => success!("==> Proceeding with installation..."),
        Ok(Installation::Exit) => {
            info!("==> Exiting...");
            return Ok(());
        }
        Err(error) => return Err(error),
    }

    match clone_repo(&paths.config, &paths.repo) {
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

    match set_wallpaper(&paths.wallpapers) {
        Ok(Wallpaper::Set) => {
            success!("==> Successfully downloaded wallpaper to ~/Documents/wallpapers");
        }
        Ok(Wallpaper::Existing) => {
            success!("==> Wallpaper already found in ~/Documents/wallpapers")
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
            success!("==> There is already a backup in ~/Documents/config_backup")
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

    match change_settings(&paths.hypr_config) {
        Ok(HyprConfig::Modified) => success!("==> Successfully modified Hypr config!"),
        Ok(HyprConfig::Default) => success!("==> Using default Hypr config"),
        Err(error) => error!("Modifying Hypr config failed", error),
    }

    match install_cli_utilities(&paths.home, &paths.config) {
        Ok(DownloadStatus::Success) => success!("==> Successfully installed CLI utilities"),
        Ok(_) => {}
        Err(error) => error!("Installing CLI utilities failed", error),
    }

    match after_install(&paths.repo) {
        Ok(()) => pause().unwrap(),
        Err(error) => error!("", error),
    }

    Ok(())
}
