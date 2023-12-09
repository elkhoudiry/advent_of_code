use std::{cmp::Ordering, ops::Index};

use crate::utils::files;

const TAG: &str = "[DAY 7-1]";
#[derive(Debug)]
struct Game {
    hand: String,
    pid: i64,
}

pub fn execute(file_path: &str) -> i64 {
    println!("{TAG} starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    let games = input
        .lines()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(item);
            println!("{TAG} line #{} result: {:#?}", index + 1, result);
            result
        })
        .collect::<Vec<_>>();

    handle_games(games)
}

fn handle_line(line: &str) -> Game {
    let (hand, pid) = line.split_once(" ").unwrap();

    Game {
        hand: hand.to_string(),
        pid: pid.parse::<i64>().unwrap(),
    }
}

fn handle_games(games: Vec<Game>) -> i64 {
    let mut fives_of_a_kind = games
        .iter()
        .filter(|game| is_five_of_a_kind(&game))
        .collect::<Vec<_>>();
    fives_of_a_kind.sort_by(|item1, item2| order(&item1.hand, &item2.hand));
    let mut fours_of_a_kind = games
        .iter()
        .filter(|game| is_four_of_a_kind(&game))
        .collect::<Vec<_>>();
    fours_of_a_kind.sort_by(|item1, item2| order(&item1.hand, &item2.hand));
    let mut full_houses = games
        .iter()
        .filter(|game| is_full_house(&game))
        .collect::<Vec<_>>();
    full_houses.sort_by(|item1, item2| order(&item1.hand, &item2.hand));
    let mut threes_of_a_kind = games
        .iter()
        .filter(|game| is_three_of_a_kind(&game))
        .collect::<Vec<_>>();
    threes_of_a_kind.sort_by(|item1, item2| order(&item1.hand, &item2.hand));
    let mut two_pairs = games
        .iter()
        .filter(|game| is_two_pair(&game))
        .collect::<Vec<_>>();
    two_pairs.sort_by(|item1, item2| order(&item1.hand, &item2.hand));
    let mut one_pairs = games
        .iter()
        .filter(|game| is_one_pair(&game))
        .collect::<Vec<_>>();
    one_pairs.sort_by(|item1, item2| order(&item1.hand, &item2.hand));
    let mut high_cards = games
        .iter()
        .filter(|game| is_high_card(&game))
        .collect::<Vec<_>>();
    high_cards.sort_by(|item1, item2| order(&item1.hand, &item2.hand));

    println!(
        "high_cards\n{:#?}\none_pairs\n{:#?}\ntwos\n{:#?}\nthrees\n{:#?}\nhouses\n{:#?}\nfours\n{:#?}\nfives\n{:#?}",
        &high_cards,
        &one_pairs,
        &two_pairs,
        &threes_of_a_kind,
        &full_houses,
        &fours_of_a_kind,
        &fives_of_a_kind,
    );

    let mut ordered: Vec<&Game> = vec![];

    for card in high_cards {
        ordered.push(card)
    }
    for card in one_pairs {
        ordered.push(card)
    }
    for card in two_pairs {
        ordered.push(card)
    }
    for card in threes_of_a_kind {
        ordered.push(card)
    }
    for card in full_houses {
        ordered.push(card)
    }
    for card in fours_of_a_kind {
        ordered.push(card)
    }
    for card in fives_of_a_kind {
        ordered.push(card)
    }

    ordered
        .iter()
        .enumerate()
        .map(|(index, game)| {
            let value = ((index as i64) + 1) * game.pid;
            println!("{TAG} game: {:#?}, value: {value}", game.hand);
            ((index as i64) + 1) * game.pid
        })
        .sum::<i64>()
}

fn is_five_of_a_kind(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    return chars.len() == 1;
}

fn is_four_of_a_kind(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();

    let mut counter = 0;

    while counter < chars.len() - 3 {
        let char = chars[counter];
        if char == chars[counter + 1] && char == chars[counter + 2] && char == chars[counter + 3] {
            return !is_five_of_a_kind(game);
        }
        counter += 1;
    }

    false
}

fn is_full_house(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    return chars.len() == 2 && !is_four_of_a_kind(game);
}

fn is_three_of_a_kind(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();

    let mut counter = 0;

    while counter < chars.len() - 2 {
        let char = chars[counter];
        if char == chars[counter + 1] && char == chars[counter + 2] {
            chars.dedup();
            return chars.len() == 3 && !is_full_house(game);
        }
        counter += 1;
    }

    false
}

fn is_two_pair(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    return chars.len() == 3 && !is_three_of_a_kind(game);
}

fn is_one_pair(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    return chars.len() == 4;
}

fn is_high_card(game: &Game) -> bool {
    let mut chars = game.hand.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    return chars.len() == 5;
}

fn order(item1: &str, item2: &str) -> core::cmp::Ordering {
    let chars1 = item1.chars().collect::<Vec<_>>();
    let chars2 = item2.chars().collect::<Vec<_>>();
    let values = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    for i in 0..chars1.len() {
        let power1 = values.into_iter().position(|c| c == chars1[i]);
        let power2 = values.into_iter().position(|c| c == chars2[i]);

        if power1 < power2 {
            return Ordering::Greater;
        } else if power1 > power2 {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}
