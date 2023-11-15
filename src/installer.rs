use crate::{
    error, info, prompt, success, tip,
    utils::{contents::get_kb_layouts, core::*},
    warning,
};
use colored::Colorize;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::{self, Command, Output},
};

pub fn installation_prompt() -> io::Result<()> {
    let mut input: String;

    tip!("This installer will copy the config files from this repo: https://github.com/3rfaan/arch-everforest\n\
          Make sure you've installed those programs to get the best experience.");
    warning!("==> This installer will modify directories inside your ~/.config directory");

    loop {
        prompt!("Do you want to proceed? [y/N]");

        input = read_input()?;

        match parse_input(&input) {
            UserInput::Yes => break,
            UserInput::No => process::exit(1),
            UserInput::Other => prompt!("==> Please enter [y]es or [n]o!"),
        }
    }

    Ok(())
}

// Check if "~/.config" directory exists. If not create a new one.
pub fn check_config_dir(config_path: &Path) -> io::Result<()> {
    info!("Checking if ~/.config directory exists...");

    fs::create_dir_all(config_path)?;

    Ok(())
}

// Clones Github repo into ~/Downloads/arch-everforest
pub fn clone_repo(repo_path: &Path) -> io::Result<DownloadStatus> {
    const REPO_URL: &str = "https://github.com/3rfaan/arch-everforest";

    info!("Cloning into https://github.com/3rfaan/arch-everforest...");

    if repo_path.exists() {
        return Ok(DownloadStatus::Existing);
    }

    let output: io::Result<Output> = Command::new("git")
        .arg("clone")
        .arg(REPO_URL) // URL to Github repository
        .arg(repo_path) // Destination path
        .output();

    match output {
        Ok(_) => Ok(DownloadStatus::Success),
        Err(error) => Err(error),
    }
}

pub fn download_wallpaper(downloads_path: &Path) -> io::Result<DownloadStatus> {
    const URL: &str =
        "https://raw.githubusercontent.com/Apeiros-46B/everforest-walls/main/close_up/flowers.png";

    info!("Downloading wallpaper into ~/Downloads");

    if downloads_path.join("flowers.png").exists() {
        return Ok(DownloadStatus::Existing);
    }

    let output: io::Result<Output> = Command::new("wget")
        .arg(URL)
        .arg("--output-document")
        .arg(downloads_path.join("flowers.png"))
        .output();

    match output {
        Ok(_) => Ok(DownloadStatus::Success),
        Err(error) => Err(error),
    }
}

pub fn set_wallpaper(downloads_path: &Path, documents_path: &Path) -> io::Result<()> {
    let wallpapers_path: PathBuf = documents_path.join("wallpapers");

    if !wallpapers_path.exists() {
        fs::create_dir_all(&wallpapers_path)?;
    }

    let src_path: PathBuf = downloads_path.join("flowers.png");
    let dest_path: PathBuf = wallpapers_path.join("flowers.png");

    if !dest_path.exists() {
        fs::copy(&src_path, dest_path)?;
    }

    if src_path.exists() {
        fs::remove_file(src_path)?;

        success!("==> Wallpaper in ~/Downloads/flowers.png has been removed");
    }

    Ok(())
}

// Delete directories and files which are not needed to moved to ~/.config directory
pub fn cleanup_repo(home_path: &Path, repo_path: &Path) -> io::Result<()> {
    let entries_to_delete: &[&str] = &["arch-everforest_short.mp4", ".git", "README.md", "zsh"];

    info!("Removing some directories and files which are not needed to be moved to ~/.config...");

    cleanup(&home_path, &repo_path, entries_to_delete)?;

    Ok(())
}

// Creates backup of all files and directories inside ~/.config and puts it inside ~/Documents/backup
pub fn create_backup(config_path: &Path, documents_path: &Path) -> io::Result<BackupStatus> {
    let backup_path: PathBuf = documents_path.join("config_backup");

    info!("Creating backup of your current ~/.config directory...");

    if backup_path.exists() {
        return Ok(BackupStatus::Existing);
    } else {
        fs::create_dir(&backup_path)?;
    }

    if let Err(error) = copy_recursively(config_path, backup_path) {
        error!(
            "Could not create backup directory at ~/Documents/backup",
            error
        );

        loop {
            prompt!("The theme can still be installed. Do you want to continue? [y/N]");

            let input: String = read_input()?;

            match parse_input(&input) {
                UserInput::Yes => return Ok(BackupStatus::NoBackup),
                UserInput::No => return Err(error),
                UserInput::Other => prompt!("==> Please enter [y]es or [n]o!"),
            }
        }
    }

    Ok(BackupStatus::Created)
}

// Copy directories from ~/Downloads/arch-everforest to ~/.config recursively
pub fn copy_config_dirs_recursively(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;

    info!("Copying directories from ~/Downloads/arch-everforest to ~/.config...");

    copy_recursively(src, dest)?;

    Ok(())
}

pub fn change_kb_layout() -> io::Result<KBLayoutStatus> {
    let mut input: String;

    loop {
        prompt!("Keyboard layout is currently set to [us]. Would you like to change it? [y/N]");

        input = read_input()?;

        match parse_input(&input) {
            UserInput::Yes => break,
            UserInput::No => return Ok(KBLayoutStatus::Default),
            UserInput::Other => prompt!("==> Please enter [y]es or [n]o!"),
        }
    }

    Ok(get_kb_layout_code()?)
}

// Helper function for `change_kb_layout()` to get valid keyboard layout code
fn get_kb_layout_code() -> io::Result<KBLayoutStatus> {
    let mut input: String;
    let kb_layouts: BTreeMap<&str, &str> = get_kb_layouts();

    loop {
        prompt!("Please enter a valid keyboard layout. Press l to see a [l]ist of available options or q to [q]uit:");

        input = read_input()?;

        match input.as_str() {
            "q" => return Ok(KBLayoutStatus::Default),
            "l" => kb_layouts
                .iter()
                .for_each(|(code, lang)| println!("{} -> {}", *code, *lang)),
            _ => {
                if let Some(_) = kb_layouts.iter().find(|(&code, _)| *code == input) {
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    Ok(update_hypr_kb_layout(&input)?)
}

// Helper function for `change_kb_layout()` to modify Hyprland config file
fn update_hypr_kb_layout(layout_code: &str) -> io::Result<KBLayoutStatus> {
    if layout_code == "us" {
        return Ok(KBLayoutStatus::Default);
    }

    let paths: Paths = Paths::build();

    // Path to Hyprland config file
    let hypr_config_file: File = File::open(&paths.hypr_config)?;
    let hypr_config_reader: BufReader<File> = BufReader::new(hypr_config_file);

    let temp_file_path: &Path = Path::new("./hyprland.conf");
    let temp_file: File = File::create(temp_file_path)?;
    let mut temp_file_stream: BufWriter<File> = BufWriter::new(temp_file);

    let old_layout: &str = "kb_layout = us";
    let new_layout: String = format!("kb_layout = {}", layout_code);

    for line in hypr_config_reader.lines() {
        let line: String = line?;

        if line.contains(old_layout) {
            let updated_line: String = line.replace(old_layout, &new_layout);

            temp_file_stream.write(updated_line.as_bytes())?;

            println!(
                "{} {}",
                "==> Changed the following line in Hypr config file:".green(),
                updated_line.trim().green().bold()
            );
        } else {
            temp_file_stream.write(line.as_bytes())?;
        }

        temp_file_stream.write(b"\n")?;
    }

    temp_file_stream.flush()?;

    if temp_file_path.exists() {
        fs::copy(temp_file_path, paths.hypr_config)?;
        fs::remove_file(temp_file_path)?;

        success!("==> Copied new Hypr config file to ~/.config/Hypr/hyprland.conf");
        success!("==> Removed temporary file");
    }

    Ok(KBLayoutStatus::Changed(layout_code.to_string()))
}

pub fn check_nvidia(hypr_config: &Path) -> io::Result<GraphicsCardStatus> {
    let mut input: String;

    loop {
        prompt!("Are you using a NVIDIA graphics card? [y/N]");

        input = read_input()?;

        match parse_input(&input) {
            UserInput::Yes => break,
            UserInput::No => return Ok(GraphicsCardStatus::Default),
            UserInput::Other => prompt!("==> Please enter [y]es or [n]o!"),
        }
    }

    Ok(change_hypr_nvidia_env(hypr_config)?)
}

// Helper function for `check_nvidia()`
fn change_hypr_nvidia_env(hypr_config: &Path) -> io::Result<GraphicsCardStatus> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#(env = .+)$").unwrap());

    let hypr_config_file: File = File::open(hypr_config)?;
    let hypr_config_reader: BufReader<File> = BufReader::new(hypr_config_file);

    let temp_file_path: &Path = Path::new("./hyprland.conf");
    let temp_file: File = File::create(temp_file_path)?;
    let mut temp_file_stream: BufWriter<File> = BufWriter::new(temp_file);

    info!("Adding Nvidia environment variables inside Hypr config");

    for line in hypr_config_reader.lines() {
        let mut line: String = line?;

        if RE.is_match(&line) {
            line = RE.replace(&line, "$1").to_string();
        }
        temp_file_stream.write(line.as_bytes())?;
        temp_file_stream.write(b"\n")?;
    }

    temp_file_stream.flush()?;

    if temp_file_path.exists() {
        fs::copy(temp_file_path, hypr_config)?;
        fs::remove_file(temp_file_path)?;

        success!("==> Copied new Hypr config file to ~/.config/Hypr/hyprland.conf");
        success!("==> Removed temporary file");
    }

    Ok(GraphicsCardStatus::Changed)
}

pub fn install_cli_utilities(home_path: &Path, config_path: &Path) -> io::Result<DownloadStatus> {
    let zsh_path: PathBuf = home_path.join(".zsh");
    let ranger_devicons_path: PathBuf = config_path.join("ranger/plugins/ranger_devicons");

    info!("Installing CLI utilies");

    if !zsh_path.exists() {
        fs::create_dir_all(&zsh_path)?;
    }

    if !zsh_path.join("zsh-autosuggestions").exists() {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/zsh-users/zsh-autosuggestions")
            .arg(&zsh_path.join("zsh-autosuggestions"))
            .output()?;

        success!("==> Successfully cloned zsh-autosuggestions");
    }

    if !zsh_path.join("zsh-syntax-highlighting").exists() {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/zsh-users/zsh-syntax-highlighting.git")
            .arg(&zsh_path.join("zsh-syntax-highlighting"))
            .output()?;

        success!("==> Successfully cloned zsh-syntax-highlighting");
    }

    if !ranger_devicons_path.exists() {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/alexanderjeurissen/ranger_devicons")
            .arg(config_path.join(ranger_devicons_path))
            .output()?;

        success!("==> Successfully cloned ranger-devicons");
    }

    Ok(DownloadStatus::Success)
}

pub fn after_install(repo_path: &Path) -> io::Result<()> {
    info!("Removing ~/Downloads/arch-everforest repo");

    if repo_path.exists() {
        fs::remove_dir_all(repo_path)?;
    }

    success!("==> Removed repo successfully".green());

    info!("Installation succeeded! 🎉");

    tip!("Tip: After this installation you have to restart Hyprland by pressing <SUPER> + <SHIFT> + E");
    tip!("Tip: To change your wallpaper change the path to another picture inside ~/.config/hypr/hyprpaper.conf");

    Ok(())
}
