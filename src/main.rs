#[macro_use]
extern crate clap;
use clap::Error as ClapError;

use std::env::current_dir;
use std::error::Error as StdError;
use std::fs::read_dir;
use std::io::{self, Error as IoError, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    let mut exit_code = 0;

    if let Err(err) = cli() {
        if let Some(clap_err) = err.downcast_ref::<ClapError>() {
            eprint!("{}", clap_err);

            io::stdout()
                .flush()
                .unwrap_or_else(|_| eprintln!("Unable to flush stdout!"));

            match clap_err.kind {
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => (),
                _ => exit_code = 1,
            }
        } else if let Some(io_err) = err.downcast_ref::<IoError>() {
            eprintln!("IO Error: {}", io_err);
            exit_code = 1;
        } else {
            exit_code = 1;
        }
    }

    exit(exit_code);
}

fn cli() -> Result<(), Box<StdError>> {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .about("A simple clone of the command tree")
        .arg(clap::Arg::with_name("DIR").help("The directory to print the contents of"))
        .get_matches_safe()?;

    let folder_path = matches.value_of("DIR").unwrap_or(".");
    let folder_path = resolve_folderpath(folder_path)?;

    tree(&folder_path, 0)?;

    Ok(())
}

/// Resolves a passed path to either a relative or absolute location.
/// If the path does not exist or refer to a folder, an `io::Error` will be returned.
fn resolve_folderpath(path: &str) -> Result<PathBuf, Box<IoError>> {
    let mut result = PathBuf::from(path);

    if !result.exists() || !result.is_dir() {
        result = current_dir()?;
        result.push(path);

        if !result.exists() || !result.is_dir() {
            return Err(IoError::new(
                io::ErrorKind::NotFound,
                "The passed path does not exist or does not refer to a folder!",
            )
            .into());
        }
    }

    Ok(result)
}

/// Recursively prints out all folders and files in the passed directory
fn tree(path: &Path, indent: usize) -> Result<(), Box<StdError>> {
    if path.is_dir() {
        for path in read_dir(path)? {
            let resolved = path?.path();
            println!(
                "{:indent$}{}",
                " ",
                resolved.to_str().unwrap(),
                indent = indent
            );
            if resolved.is_dir() {
                tree(&resolved, indent + 4)?;
            }
        }
    }
    Ok(())
}
