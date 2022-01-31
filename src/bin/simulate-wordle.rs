use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

use wordletron::*;

fn main() {
    let (answers, guesses) = load_words();
    let start = Word::try_from("raise").unwrap();

    answers.par_iter().for_each(|a| {
        let mut guess = start.clone();
        let mut answers = answers.clone();
        let mut sequence = vec![start];

        while guess != *a {
            answers.retain(|r| is_candidate(&compute_clue(a, &guess), &guess, r));
            guess = find_best_guesses(&answers, &guesses)
                .first()
                .expect("could not compute a guess")
                .clone();
            sequence.push(guess.clone());
        }
        print!("{}: ", sequence.len());
        print_sequence(&sequence);
    });
}
