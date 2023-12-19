use crate::{prompt, success, utils::contents::get_kb_layouts, KBLayout, UserInput};
use colored::Colorize;
use std::{
    collections::BTreeMap,
    fs::{self, DirEntry, FileType},
    io::{self, stdin, stdout, Read, Write},
    path::Path,
};

pub fn read_input() -> io::Result<String> {
    let mut input: String = String::new();

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase())
}

pub fn parse_input(input: &str) -> UserInput {
    match input {
        "y" | "yes" => UserInput::Yes,
        "n" | "no" | "" => UserInput::No,
        _ => UserInput::Other,
    }
}

pub fn pause() -> io::Result<()> {
    prompt!("Press Enter to close this installer... ");

    stdout().flush()?;
    let _ = stdin().read(&mut [0])?;

    Ok(())
}

pub fn copy_recursively(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry: DirEntry = entry?;
        let filetype: FileType = entry.file_type()?;

        if filetype.is_dir() {
            fs::create_dir_all(dest.as_ref().join(entry.file_name()))?;
            copy_recursively(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn cleanup<'a>(
    home_path: impl AsRef<Path>,
    repo_path: impl AsRef<Path>,
    entries_to_delete: impl AsRef<[&'a str]>,
) -> io::Result<()> {
    for entry in fs::read_dir(repo_path)? {
        let entry: DirEntry = entry?;
        let filetype: FileType = entry.file_type()?;

        if entry.file_name() == "zsh" {
            fs::copy(
                entry.path().join(".zshrc"),
                home_path.as_ref().join(".zshrc"),
            )?;

            success!("==> Copied .zshrc to ~/.zshrc before removing zsh directory");
        }

        if entries_to_delete.as_ref().contains(
            &entry
                .file_name()
                .to_str()
                .expect("Could not get directory name inside config directories"),
        ) {
            if filetype.is_dir() {
                fs::remove_dir_all(entry.path())?;
            } else {
                fs::remove_file(entry.path())?;
            }

            println!(
                "{} {}",
                "==> Successfully removed:".green(),
                entry
                    .file_name()
                    .to_str()
                    .expect("Could not get file name")
                    .green()
                    .bold()
            );
        }
    }

    Ok(())
}

pub fn get_kb_layout_code() -> io::Result<KBLayout> {
    let mut input: String;
    let kb_layouts: BTreeMap<&str, &str> = get_kb_layouts();

    loop {
        prompt!("Please enter a valid keyboard layout. Press l to see a [l]ist of available options or q to [q]uit:");

        input = read_input()?;

        match input.as_str() {
            "q" | "quit" => return Ok(KBLayout::Default),
            "l" | "list" => kb_layouts
                .iter()
                .for_each(|(code, lang)| println!("{} -> {}", *code, *lang)),
            _ => {
                if kb_layouts.iter().any(|(&code, _)| *code == input) {
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    Ok(KBLayout::Change(input))
}
