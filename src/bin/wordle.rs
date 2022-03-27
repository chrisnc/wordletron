use std::io;
use wordletron::play_wordle;

fn main() -> io::Result<()> {
    play_wordle(None)
}
