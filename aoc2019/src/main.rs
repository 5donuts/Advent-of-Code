include!(concat!(env!("OUT_DIR"), "/days_runner.rs"));

mod days;
mod intcode;

fn main() {
    run_days();
}
