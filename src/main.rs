use std::env;
use std::process;

use trash_panda::TPCommand;

fn main() {
    // First we can get the arguments passed to our program
    let arg_vec: Vec<String> = env::args().collect();

    // Create a new TPCommand, pass in arg_vec
    let mut tp_command = TPCommand::new(&arg_vec);

    // Build the wrapped command
    tp_command.build_command();

    // Run the process, if we get an error, print it to stderr
    if let Err(e) = tp_command.run() {
        eprintln!("Command Error: {e}");
        process::exit(1);
    }
}
