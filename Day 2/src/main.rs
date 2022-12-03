mod utils;
use itertools::Itertools;

#[derive(Copy, Clone)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

type Duel = (RockPaperScissors, RockPaperScissors);

#[derive(Copy, Clone)]
enum DuelResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

type ExpetedDuelWithScore = (RockPaperScissors, DuelResult);
type DuelScore = (RockPaperScissors, DuelResult);

fn letter_to_rps(letter: &str) -> RockPaperScissors {
    match letter {
        "A" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        "X" => RockPaperScissors::Rock,
        "Y" => RockPaperScissors::Paper,
        "Z" => RockPaperScissors::Scissors,
        _ => panic!("Invalid letter in input"),
    }
}

fn letter_to_duel_result(letter: &str) -> DuelResult {
    match letter {
        "X" => DuelResult::Lose,
        "Y" => DuelResult::Draw,
        "Z" => DuelResult::Win,
        _ => panic!("Invalid letter in input"),
    }
}

fn duel_to_score(duel: &Duel) -> DuelScore {
    match duel {
        (RockPaperScissors::Rock, RockPaperScissors::Rock) => ( RockPaperScissors::Rock, DuelResult::Draw),
        (RockPaperScissors::Rock, RockPaperScissors::Paper) => (RockPaperScissors::Paper, DuelResult::Win),
        (RockPaperScissors::Rock, RockPaperScissors::Scissors) => (RockPaperScissors::Scissors, DuelResult::Lose),
        (RockPaperScissors::Paper, RockPaperScissors::Rock) => ( RockPaperScissors::Rock, DuelResult::Lose),
        (RockPaperScissors::Paper, RockPaperScissors::Paper) => (RockPaperScissors::Paper, DuelResult::Draw),
        (RockPaperScissors::Paper, RockPaperScissors::Scissors) => (RockPaperScissors::Scissors, DuelResult::Win),
        (RockPaperScissors::Scissors, RockPaperScissors::Rock) => ( RockPaperScissors::Rock, DuelResult::Win),
        (RockPaperScissors::Scissors, RockPaperScissors::Paper) => (RockPaperScissors::Paper, DuelResult::Lose),
        (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => (RockPaperScissors::Scissors, DuelResult::Draw),
    }
}

fn expected_to_score(expected: &ExpetedDuelWithScore) -> DuelScore {
    match expected {
        (RockPaperScissors::Rock, DuelResult::Draw) => ( RockPaperScissors::Rock, DuelResult::Draw),
        (RockPaperScissors::Rock, DuelResult::Win) => (RockPaperScissors::Paper, DuelResult::Win),
        (RockPaperScissors::Rock, DuelResult::Lose) => (RockPaperScissors::Scissors, DuelResult::Lose),
        (RockPaperScissors::Paper, DuelResult::Lose) => ( RockPaperScissors::Rock, DuelResult::Lose),
        (RockPaperScissors::Paper, DuelResult::Draw) => (RockPaperScissors::Paper, DuelResult::Draw),
        (RockPaperScissors::Paper, DuelResult::Win) => (RockPaperScissors::Scissors, DuelResult::Win),
        (RockPaperScissors::Scissors, DuelResult::Win) => ( RockPaperScissors::Rock, DuelResult::Win),
        (RockPaperScissors::Scissors, DuelResult::Lose) => (RockPaperScissors::Paper, DuelResult::Lose),
        (RockPaperScissors::Scissors, DuelResult::Draw) => (RockPaperScissors::Scissors, DuelResult::Draw),
    }
}

fn score_to_number(duel_score: &DuelScore) -> i32 {
    match duel_score {
        (symbol, result) => *symbol as i32 + *result as i32,
    }
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut rounds: Vec<Duel> = Vec::new();
    let mut required_results: Vec<DuelScore> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {

        for line in lines {
            if let Ok(readed_line) = line {
                let splitted_line = readed_line.split_whitespace();
                let (enemy, me) = splitted_line.collect_tuple().unwrap();
                rounds.push((letter_to_rps(enemy), letter_to_rps(me)));
                required_results.push((letter_to_rps(enemy), letter_to_duel_result(me)));
            }
        }
    }

    let rounds_score: i32 = rounds.iter()
        .map(|duel| duel_to_score(&*duel))
        .map(|duel_score| score_to_number(&duel_score))
        .sum();

    let required_results_score: i32 = required_results.iter()
        .map(|duel| expected_to_score(&*duel))
        .map(|duel_score| score_to_number(&duel_score))
        .sum();

    println!("Part 1: {}", rounds_score);
    println!("Part 2: {}", required_results_score);
}