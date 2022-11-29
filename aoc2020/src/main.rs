mod days;
use utils::Day;

fn main() {
    let days = vec![Day {
        num: 1,
        p1: days::d01::part1,
        p2: days::d01::part2,
        input: "aoc2020/input/1",
    }];

    for day in days {
        day.run().unwrap();
    }
}
