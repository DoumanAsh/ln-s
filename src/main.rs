#![no_main]

mod cli;
mod symlink;

use std::{path, fs, env};

use symlink::symlink;

const FAIL_CODE: isize = 1;

#[no_mangle]
unsafe extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let args = c_ffi::Args::new(argc, argv).expect("To get function arguments");

    let args = match cli::Cli::new(args.into_iter()) {
        Ok(args) => args,
        Err(code) => std::process::exit(code),
    };

    let target = path::Path::new(&args.path);
    let mut target = match target.is_absolute() {
        true => target.to_path_buf(),
        false => match env::current_dir() {
            Ok(curr_dir) => curr_dir.join(target),
            Err(error) => {
                eprintln!("Unable to access current directory. OS Error: {}", error);
                return FAIL_CODE;
            }
        }
    };

    let link = path::Path::new(&args.link);
    let link = match link.is_absolute() {
        true => link.to_path_buf(),
        false => match env::current_dir() {
            Ok(curr_dir) => curr_dir.join(link),
            Err(error) => {
                eprintln!("Unable to access current directory. OS Error: {}", error);
                return FAIL_CODE;
            }
        }
    };

    if link.exists() {
        let meta = match link.symlink_metadata() {
            Ok(meta) => meta,
            Err(error) => {
                eprintln!("{}: Unable to access. OS Error: {}", args.link, error);
                return FAIL_CODE;
            }
        };

        if meta.file_type().is_symlink() {
            match args.force {
                true => match fs::remove_dir(&link).or_else(|_| fs::remove_dir(&link)) {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("{}: Unable to remove. OS Error: {}", args.link, error);
                        return FAIL_CODE;
                    }
                },
                false => {
                    eprintln!("{}: Already exists.", args.link);
                    return FAIL_CODE;
                }
            }
        } else {
            eprintln!("{}: is file or directory. Cannot be overwritten", args.link);
        }
    }

    if !args.abs {
        if let Some(parent) = link.parent() {
            match target.strip_prefix(parent) {
                Ok(stripped) => target = stripped.to_path_buf(),
                Err(_) => (),
            }
        }
    }

    let target_meta = match fs::metadata(&target) {
        Ok(meta) => meta,
        Err(error) => {
            eprintln!("{}: Unable to access. OS Error: {}", args.path, error);
            return FAIL_CODE;
        }
    };

    match symlink(&target, &link, target_meta) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Failed to create symlink. OS Error: {}", error);
            return FAIL_CODE;
        }
    }

    0
}
