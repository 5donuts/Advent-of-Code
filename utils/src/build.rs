use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

/// Helper function for build scripts to use to generate the
/// module & runner code to automatically load & run modules
/// for puzzle days.
pub fn generate_runner_code() -> Result<(), Box<dyn Error>> {
    let cargo_root =
        env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| "CARGO_MANIFEST_DIR is not set!")?;
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| "OUT_DIR is not set!")?;

    // Directory with all of the dXX.rs files (our script input)
    let days_path = Path::new(&cargo_root).join("src/days");

    // You should NOT generally write to any path other that `OUT_DIR`; this will cause you problems
    // if this crate is a dependency. For more information, see:
    // https://doc.rust-lang.org/cargo/reference/build-script-examples.html#code-generation
    let mod_path = Path::new(&cargo_root).join("src/days/mod.rs");
    let run_path = Path::new(&out_dir).join("days_runner.rs");

    // Actually generate those files
    let days = get_days_list(&days_path)?;
    if days.len() == 0 {
        return Err("No days found!".into());
    }
    build_days_module(&mod_path, &days)?;
    build_days_runner(&run_path, &days)?;

    // Communicate to Cargo when to re-run this script
    println!("cargo:rerun-if-changed=src/days/*.rs");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

/// Helper function for build scripts to automatically generate the
/// code to declare all of the separate [Day] modules.
fn build_days_module<P: AsRef<Path>>(out_path: P, days: &[Day]) -> io::Result<()> {
    let mod_lines: Vec<String> = days
        .iter()
        .map(|d| format!("pub mod {};", d.name()))
        .collect();
    let mod_lines = mod_lines.join("\n");
    fs::write(&out_path, mod_lines)
}

/// Helper function for build scripts to automatically generate the
/// code to generate the [Day] runners.
fn build_days_runner<P: AsRef<Path>>(out_path: P, days: &[Day]) -> io::Result<()> {
    let day_nums: Vec<String> = days.iter().map(|d| format!("{}", d.num())).collect();
    let day_nums = day_nums.join(", ");

    fs::write(
        &out_path,
        format!(
            "use macros::days;
use utils::Day;

fn run_days() {{
    let days = days!({});

    for day in days {{
      day.run().unwrap();
    }}
}}
",
            day_nums
        ),
    )
}

/// Function to find all of the `src/days/dXX.rs` files in a crate
/// for a build script to use.
fn get_days_list<P: AsRef<Path>>(days_dir: P) -> io::Result<Vec<Day>> {
    let days: Vec<_> = fs::read_dir(&days_dir)?
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
        .filter_map(|file_name| {
            if let Ok(day) = Day::try_from(file_name.as_str()) {
                Some(day)
            } else {
                None
            }
        })
        .collect();

    Ok(days)
}

struct Day((String, u8));

impl Day {
    fn name(&self) -> &str {
        &self.0 .0
    }

    fn num(&self) -> u8 {
        self.0 .1
    }
}

impl TryFrom<&str> for Day {
    type Error = Box<dyn Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let name = s.replace(".rs", "");
        let num_str = s.replace(".rs", "").replace("d", ""); // should just be a 2-digit number at this pt
        let num = num_str.parse()?;
        Ok(Self((name, num)))
    }
}
