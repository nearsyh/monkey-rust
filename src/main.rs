mod token;
mod lexer;
mod repl;

use std::io;

fn main() -> io::Result<()> {
    repl::start(io::stdin(), io::stdout())
}
