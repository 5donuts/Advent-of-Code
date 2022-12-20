use macros::days;
use utils::Day;

mod days;

fn main() {
    let days = days!(1);

    for day in days {
        day.run().unwrap();
    }
}
