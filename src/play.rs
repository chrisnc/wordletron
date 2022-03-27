use rand::prelude::*;
use std::io;

use super::*;

pub fn play_wordle(answer: Option<Word>) -> io::Result<()> {
    let (mut answers, guesses) = load_words();
    let mut guess: Word;
    let mut sequence: Vec<Word> = Vec::new();

    let answer = answer.unwrap_or(answers.choose(&mut rand::thread_rng()).unwrap().clone());

    let mut letters = String::from("abcdefghijklmnopqrstuvwxyz");

    let mut hangman = Word::try_from("_____").unwrap();

    while sequence.len() < 6 && sequence.last() != Some(&answer) {
        println!("possible answers: {}", answers.len());
        println!("available letters: {}", letters);
        println!("answer so far: {}", hangman);
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
        for (i, _) in clue.iter().enumerate() {
            if clue[i] == Black {
                letters = letters.replace(guess[i] as char, "");
            } else if clue[i] == Green {
                hangman[i] = guess[i];
            }
        }
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
