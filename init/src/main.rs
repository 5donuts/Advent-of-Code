use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use structopt::StructOpt;

mod templates;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Init",
    about = "Helper script to set up a new day of AoC puzzles"
)]
struct Opt {
    /// The year to init or update
    #[structopt(name = "year")]
    year: u16,

    /// The day to init. If this option is not set, only the year is initialized
    #[structopt(name = "day")]
    day: Option<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if !year_exists(opt.year)? {
        init_year(opt.year)?;
    }

    if let Some(day) = opt.day {
        init_day(opt.year, day)?;
    }

    Ok(())
}

// Initialize a new year
fn init_year(year: u16) -> Result<(), Box<dyn Error>> {
    if !prompt(&*format!("Year {} does not exist; create it y/N? ", year)) {
        return Err(format!("Unable to init year {} (denied)", year).into());
    }

    let cwd = std::env::current_dir()?;
    let year_name = format!("aoc{}", year);
    let crate_root = Path::new(&cwd).join(&year_name);

    // Create the directory structure
    fs::create_dir_all(&crate_root.join("src/days"))?;

    // Create src/main.rs, build.rs, & Cargo.toml
    fs::write(&crate_root.join("src/main.rs"), templates::MAIN)?;
    fs::write(&crate_root.join("build.rs"), templates::BUILD)?;
    fs::write(&crate_root.join("Cargo.toml"), templates::cargo_toml(year))?;

    // Tell the user to do the bit that's annoying to code
    eprintln!(
        "Initialized year {}; you must now add '{}' as a workspace crate.",
        year, &year_name
    );

    Ok(())
}

// Initialize a new day for the given year
fn init_day(year: u16, day: u8) -> Result<(), Box<dyn Error>> {
    if day_exists(year, day)? {
        eprintln!("Day {}/{} already exists. Exiting.", year, day);
        return Err(format!("Day {}/{} already exists!", year, day).into());
    }

    if !prompt(&*format!(
        "Day {}/{} does not exist; create it y/N? ",
        year, day
    )) {
        return Err(format!("Unable to init day {}/{} (denied)", year, day).into());
    }

    let cwd = std::env::current_dir()?;
    let year_crate = format!("aoc{}", year);
    let day_str = format!("d{}.rs", day);
    let day_path = Path::new(&cwd)
        .join(&year_crate)
        .join("src/days")
        .join(&day_str);

    Ok(fs::write(&day_path, templates::DAY)?)
}

// Check that the given year directory already exists.
// We just assume that means it's been properly initialized b/c I'm too lazy
// to add any additional checks.
fn year_exists(year: u16) -> Result<bool, Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let year_crate = format!("aoc{}", year);

    Ok(fs::read_dir(&cwd)?
        .into_iter()
        .filter_map(|entry| {
            if entry.is_err() {
                return None;
            }

            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() {
                return None;
            }

            let path = path.file_name().unwrap().to_str().unwrap();
            if path == year_crate {
                Some(path.to_string())
            } else {
                None
            }
        })
        .count()
        > 0)
}

// Check that the given day directory already exists.
// We just assume that means it's been properly initialized b/c I'm lazy.
// Additionally, we assume the year must exist when this function is called.
fn day_exists(year: u16, day: u8) -> Result<bool, Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    let day_name = format!("d{}.rs", day);
    let crate_name = format!("aoc{}", year);
    let days_dir = Path::new(&cwd).join(&crate_name).join("src/days");

    Ok(fs::read_dir(&days_dir)?
        .into_iter()
        .filter_map(|entry| {
            if entry.is_err() {
                return None;
            }

            let entry = entry.unwrap();
            let path = entry.path();

            // Skip all directories
            if !path.is_file() {
                return None;
            }

            // Grab the filename instead of the whole path
            if let None = path.file_name() {
                return None;
            }
            let path = path.file_name().unwrap().to_str().unwrap();

            // Only grab the dXX.rs files
            if path.starts_with("d") && path.ends_with(".rs") {
                Some(path.to_string())
            } else {
                None
            }
        })
        .filter(|file_name| file_name == &day_name)
        .count()
        > 0)
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
