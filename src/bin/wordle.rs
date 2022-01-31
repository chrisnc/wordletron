use std::io;
use wordletron::*;
use rand::prelude::*;

fn main() -> io::Result<()> {
    let (mut answers, guesses) = load_words();
    let mut guess: Word;
    let mut sequence: Vec<Word> = Vec::new();

    let answer = answers.choose(&mut rand::thread_rng()).unwrap().clone();

    while sequence.len() < 6 && sequence.last() != Some(&answer) {
        println!("possible answers: {}", answers.len());
        match get_guess() {
            Ok(g) => guess = g,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
        if !guesses.contains(&guess) {
            println!("not a valid word!");
            continue;
        }
        let clue = compute_clue(&answer, &guess);
        println!("clue: {}", clue);
        answers.retain(|a| is_candidate(&clue, &guess, a));
        sequence.push(guess);
    }

    if sequence.last() == Some(&answer) {
        print!("solved in {} guesses: ", sequence.len());
        print_sequence(&sequence);
        println!("{}", clue_grid(&sequence, &answer));
    } else {
        println!("the answer was: {}", answer);
    }

    Ok(())
}
