mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("Day 1.1: {}", day1::run(day1::DATA, day1::part1));
    println!("Day 1.2: {}", day1::run(day1::DATA, day1::part2));
    println!("Day 2.1: {}", day2::run(day2::DATA, day2::part1));
    println!("Day 2.2: {}", day2::run(day2::DATA, day2::part2));
    println!("Day 3.1: {}", day3::run(day3::DATA, day3::part1));
    println!("Day 3.2: {}", day3::run(day3::DATA, day3::part2));
    println!("Day 4.1: {}", day4::run(day4::DATA, day4::part1));
    println!("Day 4.2: {}", day4::run(day4::DATA, day4::part2));
    println!("Day 5.1: {}", day5::run(day5::DATA, day5::part1));
    println!("Day 5.2: {}", day5::run(day5::DATA, day5::part2));
    println!("Day 6.1: {}", day6::run(day6::DATA, day6::part1));
    println!("Day 6.2: {}", day6::run(day6::DATA, day6::part2));
}
