use std::io;

use rpassword::prompt_password;

use wordletron::*;

fn main() -> io::Result<()> {
    play_wordle(Some(loop {
        if let Ok(solution) = Word::try_from(prompt_password("solution: ")?) {
            break solution;
        } else {
            println!("solution must be 5 letters");
        }
    }))
}
