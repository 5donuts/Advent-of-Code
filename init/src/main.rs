//! Binary workspace crate to simplify initializing additional workspace crates for challenge
//! years, day solvers, and fetching & caching puzzle inputs.

use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

mod templates;

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize a new workspace crate for a puzzle year and/or a new day runner for a new day
    /// within that year.
    New {
        /// The puzzle year to initialize.
        year: u16,

        /// The day to initialize within the specified year; if omitted, only initialize the
        /// workspace crate.
        day: Option<u8>,
    },

    /// Fetch your input to a particular puzzle.
    Fetch {
        /// The year of the puzzle for which to fetch the input.
        year: u16,

        /// The day of the puzzle for which to fetch the input; if omitted, fetch & cache all
        /// inputs required for the defined day runners (i.e., any present `src/days/dXX.rs`
        /// files).
        day: Option<u8>,

        /// Path to the file containing your session cookie value.
        #[clap(short, long, value_parser, default_value = ".session-cookie")]
        session_cookie_path: PathBuf,
    },
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.cmd {
        Command::New { year, day } => init(year, day),
        Command::Fetch {
            year,
            day,
            session_cookie_path,
        } => fetch(year, day, &session_cookie_path),
    }?;

    Ok(())
}

fn init(year: u16, day: Option<u8>) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let crate_name = format!("aoc{year}");
    let ws_crate_root = Path::new(&cwd).join(&crate_name);

    // Check if the specified year already has a workspace crate. If it does not exist, prompt the
    // user to create it. If it does exist, proceed to the optional day initialization step. This
    // assumes that should such a directory exist, it _must_ be properly configured which may or
    // may not be the case.
    if ws_crate_root.exists() && ws_crate_root.is_dir() {
        println!("Workspace crate '{crate_name}' already exists.");
    } else {
        if !prompt(&*format!(
            "Create workspace crate {crate_name} for puzzle year {year} y/N? "
        )) {
            println!("Ok; exiting.");
            return Ok(());
        }

        // Create the workspace crate directory structure
        fs::create_dir_all(&ws_crate_root.join("src/days"))?;
        fs::create_dir_all(&ws_crate_root.join("input"))?;

        // Write common workspace crate files from templates
        fs::write(&ws_crate_root.join("src/main.rs"), templates::MAIN)?;
        fs::write(&ws_crate_root.join("build.rs"), templates::BUILD)?;
        fs::write(
            &ws_crate_root.join("Cargo.toml"),
            templates::cargo_toml(year),
        )?;

        // Instruct user to update the root workspace definition
        // TODO: automate this bit, too
        println!("Initialized workspace crate '{crate_name}'; you must now add it to Cargo.toml as a workspace crate.");
    }

    if let Some(day) = day {
        let day_name = format!("d{day:0>2}.rs");
        let days_dir = Path::new(&cwd).join(&ws_crate_root).join("src/days");

        // Check if the specified day already exists. If it does, exit with an error. Otherwise, create
        // it.
        let day_runner_path = days_dir.join(&day_name);
        if day_runner_path.exists() && day_runner_path.is_file() {
            return Err(
                format!("Day runner already present: {crate_name}/src/days/{day_name}").into(),
            );
        }

        if !prompt(&*format!(
            "Create day runner '{day_name}' for puzzle year {year} y/N? "
        )) {
            println!("Ok; exiting");
            return Ok(());
        }

        fs::write(&day_runner_path, templates::DAY)?;
    }

    Ok(())
}

fn fetch(year: u16, day: Option<u8>, session_cookie_path: &Path) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn prompt(p: &str) -> bool {
    print!("{}", p);
    let _ = stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    // we only care about the first character
    let s = s.to_lowercase().chars().next().unwrap();
    if s != 'y' && s != 'n' {
        eprintln!("Unrecognized input; expected 'y' or 'n'.");
    }

    s == 'y'
}
