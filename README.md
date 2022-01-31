# Wordletron

This is a Rust crate for working with Wordle. It includes programs for
playing the game, solving a game interactively, and simulating all games
using the same strategy the interactive solver uses.

The solver is not very sophisticated; it just suggests guesses that minimize
the maximum number of answers that would remain after making a given guess,
over all answers that are still possible with the clues so far. If any of the
guesses that are best by this criterion are also in the set of remaining
answers, it will suggest only these.

### Installing

Run `cargo install wordletron`.

### Playing wordle

Just run `wordle`.

### Solving wordle

Run `solve-wordle`. Input your guesses as prompted, and input your clues
using the letters `b` for black, `y` for yellow, and `g` for green
(case-insensitive), in the order they appear for the most recent guess. The
program will show the answers that remain and the best guesses to make next
to reduce this set by as much as possible in the worst case.
