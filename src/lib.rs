extern crate structopt;

use std::error::Error;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub enum ANSIColor {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    RESET,
}

impl ANSIColor {
    pub fn as_string(&self) -> &str {
        match self {
            &ANSIColor::BLACK => "\u{001B}[0;30m",
            &ANSIColor::RED => "\u{001B}[0;31m",
            &ANSIColor::GREEN => "\u{001B}[0;32m",
            &ANSIColor::YELLOW => "\u{001B}[0;33m",
            &ANSIColor::BLUE => "\u{001B}[0;34m",
            &ANSIColor::MAGENTA => "\u{001B}[0;35m",
            &ANSIColor::CYAN => "\u{001B}[0;36m",
            &ANSIColor::WHITE => "\u{001B}[0;37m",
            &ANSIColor::RESET => "\u{001B}[0;0m",
        }
    }
}

fn visit_dirs(
    dir: &Path,
    depth: usize,
    level: usize,
    prefix: String,
    colorize: bool,
    show_all: bool,
) -> io::Result<()> {
    if (level != 0) & (depth == level) {
        return Ok(());
    }

    if dir.is_dir() {
        let entry_set = fs::read_dir(dir)?; // contains DirEntry
        let mut entries = entry_set
            .filter_map(|v| match v.ok() {
                Some(v) => {
                    if show_all {
                        return Some(v);
                    } else {
                        if v.file_name().to_str()?.starts_with(".") {
                            return None;
                        } else {
                            Some(v)
                        }
                    }
                }
                None => None,
            })
            .collect::<Vec<_>>();
        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

        for (index, entry) in entries.iter().enumerate() {
            let path = entry.path();

            if index == entries.len() - 1 {
                println!("{}└── {}", prefix, color_output(colorize, &path)?);
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.clone() + "    ";
                    visit_dirs(&path, depth, level, prefix_new, colorize, show_all)?
                }
            } else {
                println!("{}├── {}", prefix, color_output(colorize, &path)?);
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.clone() + "│   ";
                    visit_dirs(&path, depth, level, prefix_new, colorize, show_all)?
                }
            }
        }
    }
    Ok(())
}

fn is_executable(path: &Path) -> bool {
    let metadata = match fs::symlink_metadata(&path) {
        Ok(value) => value,
        Err(_err) => return false,
    };

    metadata.permissions().mode() & 0o111 != 0
}

fn color_output(colorize: bool, path: &Path) -> io::Result<String> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let symlink = match fs::read_link(path) {
        Ok(v) => v,
        Err(_err) => PathBuf::new(),
    };

    let print_name;
    if !symlink.to_str().unwrap().is_empty() {
        print_name = format!("{} -> {}", filename, symlink.to_str().unwrap());
    } else {
        print_name = filename.to_string();
    }

    match colorize {
        true => {
            if path.is_dir() {
                Ok(format!(
                    "{}{}{}",
                    ANSIColor::YELLOW.as_string(),
                    print_name,
                    ANSIColor::RESET.as_string()
                ))
            } else if is_executable(&path) {
                Ok(format!(
                    "{}{}{}",
                    ANSIColor::GREEN.as_string(),
                    print_name,
                    ANSIColor::RESET.as_string()
                ))
            } else {
                Ok(format!(
                    "{}{}{}",
                    ANSIColor::MAGENTA.as_string(),
                    print_name,
                    ANSIColor::RESET.as_string()
                ))
            }
        }
        false => Ok(format!("{}", print_name)),
    }
}

pub fn run(show_all: bool, colorize: bool, level: usize, dir: &Path) -> Result<(), Box<Error>> {
    visit_dirs(&dir, 0, level, String::from(""), colorize, show_all)?;
    Ok(())
}
