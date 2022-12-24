use macros::days;
use utils::Day;

mod days;

fn main() {
    let days = days!(1, 2, 3, 4, 5, 6);

    for day in days {
        day.run().unwrap();
    }
}
