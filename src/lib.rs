use std::io::{self, stdin, stdout, Write};

use itertools::join;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod word;
pub use word::*;

mod clue;
pub use clue::*;

pub const N: usize = 5;

fn word_vec_from_str(s: &str) -> Vec<Word> {
    s.lines().filter_map(|l| l.try_into().ok()).collect()
}

pub fn load_words() -> (Vec<Word>, Vec<Word>) {
    let mut answers = word_vec_from_str(include_str!("answers.txt"));
    let mut guesses = word_vec_from_str(include_str!("guesses.txt"));
    guesses.extend(answers.iter().cloned());
    answers.sort();
    guesses.sort();
    (answers, guesses)
}

pub fn get_guess() -> io::Result<Word> {
    print!("guess: ");
    stdout().flush()?;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    line.trim()
        .try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn get_clue() -> io::Result<Clue> {
    print!("clue: ");
    stdout().flush()?;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    line.trim()
        .chars()
        .filter_map(|c| ClueColor::try_from(c).ok())
        .collect::<Vec<ClueColor>>()
        .as_slice()
        .try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn print_sequence(seq: &Vec<Word>) {
    println!("{}", join(seq, ", "));
}

pub fn is_candidate(clue: &Clue, guess: &Word, word: &Word) -> bool {
    let mut w: Word = word.clone();

    // Check all green clues.
    for (i, _) in clue.iter().enumerate().filter(|&(_, &c)| c == Green) {
        if guess[i] != w[i] {
            return false;
        }
        w[i] = 0;
    }

    // Check all yellow clues.
    for (i, _) in clue.iter().enumerate().filter(|&(_, &c)| c == Yellow) {
        // Find another instance of the letter in the candidate word.
        match w.iter().position(|&chr| chr == guess[i]) {
            Some(j) => {
                if j == i {
                    // If the letter is in the exact position as in the guess, the clue
                    // should have been green, so this word is not a candidate.
                    return false;
                } else {
                    // There is another instance, so clear it so it can't be used for another clue.
                    w[j] = 0;
                }
            }
            // There are no more instances of this letter in the word.
            None => return false,
        }
    }

    // Check all black clues.
    for (i, _) in clue.iter().enumerate().filter(|&(_, &c)| c == Black) {
        if w.iter().position(|&chr| chr == guess[i]).is_some() {
            return false;
        }
    }

    true
}

pub fn compute_clue(answer: &Word, guess: &Word) -> Clue {
    let mut a = answer.clone();
    let mut g = guess.clone();
    let mut clue = Clue::from([Black; N]);

    /*
     * Emit a green clue for each matching letter and delete the letter from the guess and answer
     * vectors so they can't be used for other clues.
     */
    for (pos, (gc, ac)) in g
        .iter_mut()
        .zip(a.iter_mut())
        .enumerate()
        .filter(|(_, (gc, ac))| *gc == *ac)
    {
        clue[pos] = Green;
        *gc = 0;
        *ac = 0;
    }

    /*
     * For each unmarked guess letter, find the first position of that letter in the answer, emit a
     * yellow clue, and mark both the guess and answer letter so they can't be used for other
     * clues.
     */
    for (pos, gc) in g.iter().enumerate().filter(|&(_, gc)| *gc != 0) {
        if let Some(ac) = a.iter_mut().find(|ac| *gc == **ac) {
            clue[pos] = Yellow;
            *ac = 0;
        }
    }

    /*
     * The remaining elements of clue remain Black.
     */

    clue
}

pub fn find_best_guesses(answers: &[Word], guesses: &[Word]) -> Vec<Word> {
    let mut guess_scores: Vec<(Word, usize)> = guesses
        .par_iter()
        .cloned()
        .map(|g| {
            (
                g,
                answers
                    .par_iter()
                    .map(|a| {
                        let clue = compute_clue(a, &g);
                        answers
                            .par_iter()
                            .filter(|w| is_candidate(&clue, &g, w))
                            .count()
                    })
                    .max()
                    .expect("no answers left"),
            )
        })
        .collect();

    guess_scores.sort_by_key(|&(_, s)| s);

    if let Some((_, best_score)) = guess_scores.first() {
        // Need to drop best_score reference before calling .retain.
        let best_score = best_score.clone();
        guess_scores.retain(|(_, s)| *s == best_score);
    }

    let mut best_guesses: Vec<Word> = guess_scores.iter().map(|&(w, _)| w).collect();

    // If any best guesses are in the answer set, eliminate best guesses that aren't.
    if best_guesses.iter().any(|g| answers.contains(g)) {
        best_guesses.retain(|g| answers.contains(g));
    }
    best_guesses
}

pub fn clue_grid(sequence: &Vec<Word>, answer: &Word) -> String {
    sequence
        .iter()
        .map(|w| compute_clue(answer, w))
        .map(|c| {
            let s: String = (&c).into();
            s
        })
        .collect::<Vec<String>>()
        .join("\n")
}
