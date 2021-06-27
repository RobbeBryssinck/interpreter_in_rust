pub mod token;
pub mod lexer;
pub mod repl;

use repl::start;

/*
TODO:
* Proper logging
*/

fn main() {
    start();
}
