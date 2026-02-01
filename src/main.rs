use std::env;

use trash_panda::TPCommand;
use trash_panda::Tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     *   The general top-level flow of the program should go something like this:
     *   1. We collect the arguments passed to this program.
     *   2. We determine what command the user is trying to perform.
     *   3. We create a sequence of commands that provides our service for the user.
     *       Example:
     *           The user calls: tp rm -rf ./some_directory
     *           We collect the arguments: {tp, rm , -rf, ./some_directory}
     *           We determine the command the user intends to perform: rm -rf ./some_directory
     *           We create our own sequence of commands:
     *               cp -r ./some_directory /our/trash/directory
     *               rm -rf ./some_directory
     *               OR simply:
     *               mv ./some_directory /our/trash/directory
     *   4. We log this action to a file.
     * */

    // First we can get the arguments passed to our program
    let arg_vec: Vec<String> = env::args().collect();

    // Let's try to get the HOME environment variable
    let home_dir = std::env::var("HOME")?;

    // Create a Tokenizer to process the arg_vec
    let mut tokenizer: Tokenizer = Tokenizer::new(&arg_vec);

    // Tokenize the input
    tokenizer.gen_tokens();

    // Pass tokens into a new TPCommand struct, and run
    let tp = TPCommand::new(&tokenizer.get_tokens(), &home_dir);
    let status = tp.run()?;
    println!("Command finished with: {}", status);
    Ok(())
}
