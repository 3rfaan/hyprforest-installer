use std::path::PathBuf;

pub enum UserInput {
    Yes,
    No,
    Other,
}

pub enum Installation {
    Proceed,
    Exit,
}

pub enum BackupStatus {
    Created,
    Existing,
    NoBackup,
}

pub enum DownloadStatus {
    Success,
    Existing,
}

pub enum HyprConfig {
    Modified,
    Default,
}

pub enum KBLayout {
    Change(String),
    Default,
}

pub struct Paths {
    pub home: PathBuf,
    pub config: PathBuf,
    pub documents: PathBuf,
    pub repo: PathBuf,
    pub hypr_config: PathBuf,
    pub wallpapers: PathBuf,
}

impl Paths {
    pub fn build() -> Self {
        Self {
            home: dirs::home_dir().expect("Cannot get ~ path"),
            config: dirs::config_dir().expect("Cannot get ~/.config path"),
            documents: dirs::document_dir().expect("Cannot get ~/Documents path"),
            repo: dirs::download_dir()
                .expect("Cannot get ~/Downloads path")
                .join("dotfiles"),
            hypr_config: dirs::config_dir()
                .expect("Cannot get ~/.config path")
                .join("hypr/hyprland.conf"),
            wallpapers: dirs::document_dir()
                .expect("Cannot get ~/Documents path")
                .join("wallpapers"),
        }
    }
}
