mod day1;
mod day2;

fn main() {
    println!("Day 1.1: {}", day1::run(day1::DATA, day1::part1));
    println!("Day 1.2: {}", day1::run(day1::DATA, day1::part2));
    println!("Day 2.1: {}", day2::run(day2::DATA, day2::part1));
    println!("Day 2.2: {}", day2::run(day2::DATA, day2::part2));
}
