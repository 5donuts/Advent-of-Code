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
    let cwd = std::env::current_dir()?;
    let crate_name = format!("aoc{year}");
    let ws_crate_root = Path::new(&cwd).join(&crate_name);
    let input_dir = ws_crate_root.join("input");

    // Ensure the path to the session cookie file exists, then load the cookie.
    if !(session_cookie_path.exists() && session_cookie_path.is_file()) {
        return Err(format!(
            "No such session cookie file: {}",
            session_cookie_path
                .to_str()
                .unwrap_or("err: path contains non-unicode data")
        )
        .into());
    }

    let session_cookie = fs::read_to_string(session_cookie_path)?;

    // Check if the workspace crate exists; if not, exit with an error
    if !(ws_crate_root.exists() && ws_crate_root.is_dir()) {
        return Err(format!("Workspace crate '{crate_name}' does not exist!").into());
    }

    // Check if the input directory for the workspace crate exists; if not, create it.
    if !(input_dir.exists() && input_dir.is_dir()) {
        println!("Creating {crate_name}/input directory");
        fs::create_dir(&input_dir)?;
    }

    // If a day was specified, check to see if the input file for that day already exists.
    // If not, attempt to download it using the specified session cookie.
    if let Some(day) = day {
        let day_input_file = input_dir.join(format!("{day}"));

        if day_input_file.exists() && day_input_file.is_file() {
            println!("Input file {crate_name}/input/{day} already exists; skipping download.");
        } else {
            fetch_day_input(year, day, &session_cookie, &day_input_file)?;
        }
    }
    // If no day was specified, check the list of present day runners (i.e.,
    // `<crate>/src/days/dXX.rs` files). For each of those, check if the corresponding input file
    // is present. If not, attempt to download it using the specified session cookie.
    else {
        let days_mod_path = ws_crate_root.join("src/days");
        let day_runner_regex = regex::Regex::new(r"^d\d\d\.rs$")?;
        let mut day_runners: Vec<_> = fs::read_dir(&days_mod_path)?
            .into_iter()
            .filter_map(|entry| {
                if entry.is_err() {
                    return None;
                }

                let path = entry.unwrap().path();
                if !path.is_file() {
                    return None;
                }

                let fname = path.file_name().unwrap().to_str().unwrap();
                day_runner_regex.captures(fname).and_then(|cap| {
                    cap.get(0)
                        // Pull out just the `XX` from a `dXX.rs` filename
                        .and_then(|cap| Some(cap.as_str()[1..=2].parse::<u8>().unwrap()))
                })
            })
            .collect();
        day_runners.sort_unstable();

        println!("Found {} day runners for year {year}", day_runners.len());

        for day in day_runners {
            let day_input_file = input_dir.join(format!("{day}"));
            if day_input_file.exists() && day_input_file.is_file() {
                println!("Input file {crate_name}/input/{day} already exists; skipping download.");
                continue;
            }

            fetch_day_input(year, day, &session_cookie, &day_input_file)?;

            // Give a pause to at least try to be nice to the AoC servers
            std::thread::sleep(std::time::Duration::from_secs(3));
        }

        println!("done!");
    }

    Ok(())
}

/// Download the input file for a single day and write it to the disk
fn fetch_day_input(
    year: u16,
    day: u8,
    session_cookie: &str,
    output_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let download_url = format!("https://adventofcode.com/{year}/day/{day}/input");
    if !prompt(&*format!("Fetching '{download_url}'. Proceed y/N? ")) {
        println!("Ok; exiting");
        return Ok(());
    }

    let cookie_jar = {
        let cookie = format!("session={session_cookie}; Domain=adventofcode.com");
        let url = "https://adventofcode.com/".parse::<reqwest::Url>()?;

        let jar = reqwest::cookie::Jar::default();
        jar.add_cookie_str(&cookie, &url);

        jar
    };

    let user_agent = format!(
        "Rust/{} (reqwest) GitHub/5donuts/Advent-of-Code Input Fetcher {}",
        env!("RUSTC_VERSION"),
        env!("CARGO_PKG_VERSION")
    );

    let request = reqwest::blocking::Client::builder()
        .cookie_provider(std::sync::Arc::new(cookie_jar))
        .build()?
        .get(download_url)
        .header(reqwest::header::USER_AGENT, user_agent);

    let response = request.send()?;

    if !response.status().is_success() {
        return Err(format!("Error: {}", response.status().as_str()).into());
    }

    println!("Got {} bytes", response.content_length().unwrap());

    let input_text = response.text()?;
    fs::write(&output_path, input_text)?;

    println!("Wrote {}", output_path.to_str().unwrap());

    Ok(())
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
