use std::fs::File;
use std::io::{BufRead, BufReader};

///
/// There are two sets of list per line. winning cards (left of the |) and your cards (right of the |)
/// need to make a match. collection vs collection, how many exist
/// each match after the first is a double of the value. I.e. 2 ^ (match - 1)
/// Finally, add each lines result

// const LAST_LINE:usize = 6;
const LAST_LINE:usize = 218;

pub fn calculate_day_four(file: File, debug: bool) -> u64 {
    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;
    let mut counter: usize = 0;
    let scratch_cards: &mut [u32; LAST_LINE] = &mut [1; LAST_LINE];

    file_lines.flatten().for_each(|line| {
        if debug { println!("{:?} Starting scs : {:?}", &counter, &scratch_cards); }
        counter = line_result(line, counter, scratch_cards, debug);
    });

    for sc in scratch_cards {
        final_result += *sc as u64;
    }

    return final_result;
}

fn line_result(current_line: String, counter: usize,
               scratch_cards: &mut [u32; LAST_LINE], debug: bool) -> usize {
    let cards_result: Vec<&str> = current_line.split("|").collect();
    let winning_cards: Vec<&str> = cards_result[0].split(":").collect();

    let winning_list: Vec<&str> = winning_cards[1].split_whitespace().collect();
    let elves_list: Vec<&str> = cards_result[1].split_whitespace().collect();

    //if debug { println!("Winning cards: {:?}", &winning_list)}
    //if debug { println!("Elves cards: {:?}", &elves_list)}

    return compute_line_score(winning_list, elves_list, counter, scratch_cards, debug);
}

///
/// Card 1: 1 ( 4 matches)  1
/// Card 2: 1 ( 2 matches)  1 + 1
/// Card 3: 1 ( 2matches)   1 + 1 + 1 + 1
/// Card 4: 1 ( 1 match)    1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
/// Card 5: 1 ( 0 matches)  1 + 1 + 1 + 1 + 1 + 1
/// Card 6: 1 (0 matches)   1 +
///
/// 1 = 1           1
/// 2 = 1 + 1       2
/// 3 = 1 + 1 + 1   3
/// 4 = 1 + 1 + 1   3
/// 5 = 1 + 1 + 1   3
/// 12

fn compute_line_score(winning_list:Vec<&str>, elves_list:Vec<&str>, starting_index: usize,
                      scratch_cards: &mut [u32; LAST_LINE], debug: bool) -> usize {
    let mut total_matches: usize = 0;

    winning_list.iter().into_iter().for_each(|wl| {
        if elves_list.contains(wl) {
            total_matches +=1;
            if (starting_index + total_matches) < LAST_LINE {
                scratch_cards[starting_index + total_matches] += (scratch_cards[starting_index]);
            }
        }
    });

    if debug { println!("Scratch Cards = {:?} | total matches {:?}", &scratch_cards, &total_matches); }

    return (starting_index + 1);
}