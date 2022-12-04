use core::str;
use log::{ debug, info };
use test_log::test;

fn build_elves(calories: &String) -> Vec<u32> {
    debug!("Input calories: {:?}", calories);

    let mut elves: Vec<u32> = vec![0];
    let mut i = 0;

    for l in calories.lines() {
        if str::is_empty(l) {
            i = i + 1;
            elves.push(0);
        } else {
            elves[i] = elves[i] + l.parse::<u32>().unwrap();
        }
    }
    debug!("Elves: {:?}", elves);

    elves
}

pub fn find_biggest_elf(calories: &String) -> u32 {
    let elves = build_elves(calories);

    let mut max: u32 = 0;

    for i in elves.iter() {
        if i > &max {
            max = *i;
        }
    }

    debug!("Max elf: {}", max);

    *elves.iter().max().unwrap()
}

pub fn find_big_three_elf(calories: &String) -> u32 {
    let mut elves: Vec<u32> = build_elves(calories);
    elves.sort();

    debug!("Elves ordered {:?}", elves);

    let mut sum = 0;
    sum += elves.pop().unwrap();
    sum += elves.pop().unwrap();
    sum += elves.pop().unwrap();

    debug!("Biggest three sum: {}", sum);

    sum
}

#[test]
fn biggest_elf() {
    let contents = std::fs
        ::read_to_string("data/01-test")
        .expect("Something went wrong reading the file");

    let biggest_elf = find_biggest_elf(&contents);

    assert!(biggest_elf == 24000);
}

#[test]
fn biggest_three_elf() {
    let contents = std::fs
        ::read_to_string("data/01-test")
        .expect("Something went wrong reading the file");

    let biggest_elf = find_big_three_elf(&contents);

    assert!(biggest_elf == 45000);
}