use std::io;

use rpassword::prompt_password;

use wordletron::*;

fn main() -> io::Result<()> {
    let solution = prompt_password("solution: ")?;
    play_wordle(Word::try_from(solution).ok())
}
