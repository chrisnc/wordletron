use wordletron::*;

fn main() {
    let (answers, guesses) = load_words();
    println!("best guesses: ");
    print_sequence(&find_best_guesses(&answers, &guesses));
}
