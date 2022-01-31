use std::io;
use wordletron::*;

fn main() -> io::Result<()> {
    let (mut answers, mut guesses) = load_words();
    let mut sequence: Vec<Word> = Vec::new();
    let mut guess = Word([0; N]);

    while answers.len() > 1 {
        if sequence.len() > 0 {
            println!("best guesses: ");
            print_sequence(&find_best_guesses(&answers, &guesses));
        }
        match get_guess() {
            Ok(g) => guess = g,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
        let clue: Clue;
        match get_clue() {
            Ok(c) => clue = c,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
        answers.retain(|a| is_candidate(&clue, &guess, a));
        guesses.retain(|a| is_candidate(&clue, &guess, a));
        println!("answers remaining: {}", answers.len());
        print_sequence(&answers);
        sequence.push(guess);
    }

    let answer = answers.first().expect("no answer found").clone();

    // If the last guess wasn't the answer, print another round.
    if guess != answer {
        println!("guess: {}", answer);
        println!("clue: ggggg");
        sequence.push(answer);
    }

    print!("solved in {} guesses: ", sequence.len());
    print_sequence(&sequence);
    println!("{}", clue_grid(&sequence, &answer));

    Ok(())
}
