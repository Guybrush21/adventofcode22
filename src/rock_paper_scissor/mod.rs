use core::str;
use log::{ debug, info };
use test_log::test;

/// points
/// win 6 draw 3 loose 0
/// rock 1 paper 2 scissor 3
/// abc xyz rock paper scissor

pub fn calculate_strategy_result(strategy: &String) -> i32 {
    let (mut me, mut elf) = (0, 0);

    for l in strategy.lines() {
        let mut iter = l.split_whitespace();
        let elfPlay = normalize(iter.next().unwrap());
        let myPlay = normalize(iter.next().unwrap());

        me += calculate_move_value(myPlay);
        elf += calculate_move_value(elfPlay);

        me += play_result(myPlay, elfPlay);
        elf += play_result(elfPlay, myPlay);
    }

    me
}

pub fn calculate_better_strategy_result(strategy: &String) -> i32 {
    let (mut me, mut elf) = (0, 0);

    for l in strategy.lines() {
        debug!("=== Line of Play: {} ===", l);

        let mut iter = l.split_whitespace();
        let opponent_move = normalize(iter.next().unwrap());
        let my_move = normalize_strategy(opponent_move, iter.next().unwrap());

        debug!("Opponent Move: {}", opponent_move);
        debug!("My Move: {}", my_move);

        me += calculate_move_value(my_move);
        elf += calculate_move_value(opponent_move);

        debug!("My result after considering move: {}", me);
        debug!("Opponent result after considering move: {}", me);

        me += play_result(my_move, opponent_move);
        elf += play_result(opponent_move, my_move);

        debug!("My result after result: {}", me);
        debug!("Opponent result after result: {}", me);
    }
    debug!("Better strategy result: {}", me);
    me
}

fn normalize_strategy<'a>(opponent_move: &'a str, result: &'a str) -> &'a str {
    if result == "X" {
        //LOOSE{
        if opponent_move == "R" {
            return "S";
        }
        if opponent_move == "P" {
            return "R";
        }
        if opponent_move == "S" {
            return "P";
        }
    }

    if result == "Z" {
        //WIN
        if opponent_move == "R" {
            return "P";
        }
        if opponent_move == "P" {
            return "S";
        }
        if opponent_move == "S" {
            return "R";
        }
    }

    if result == "Y" {
        //DRAW
        return opponent_move;
    }
    ""
}

fn normalize(value: &str) -> &str {
    match value {
        "A" | "X" => "R",
        "B" | "Y" => "P",
        "C" | "Z" => "S",
        _ => "",
    }
}

fn play_result(first: &str, second: &str) -> i32 {
    let mut result = 0;
    if first == second {
        result = 3;
    }
    if first == "R" && second == "P" {
        result = 0;
    }
    if first == "R" && second == "S" {
        result = 6;
    }
    if first == "P" && second == "S" {
        result = 0;
    }
    if first == "P" && second == "R" {
        result = 6;
    }
    if first == "S" && second == "R" {
        result = 0;
    }
    if first == "S" && second == "P" {
        result = 6;
    }

    result
}

fn calculate_move_value(elf_play: &str) -> i32 {
    match elf_play {
        "R" | "A" | "X" => 1,
        "P" | "B" | "Y" => 2,
        "S" | "C" | "Z" => 3,
        _ => 0,
    }
}

#[test]
fn strategy_result() {
    let contents = std::fs
        ::read_to_string("data/02-test")
        .expect("Something went wrong reading the file");

    let strategy_result = calculate_strategy_result(&contents);

    assert!(strategy_result == 15);
}

#[test]
fn better_strategy_result() {
    let contents = std::fs
        ::read_to_string("data/02-test")
        .expect("Something went wrong reading the file");

    let strategy_result = calculate_better_strategy_result(&contents);

    assert!(strategy_result == 12);
}