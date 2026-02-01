use std::env;
use std::error::Error;
use std::process;
use std::process::Command;

fn main() {
    // First we can get the arguments passed to our program
    let arg_vec: Vec<String> = env::args().collect();

    // Now we can spawn a new process from the command line
    // let mut echo_hello = Command::new("echo");

    let mut tp_command = TPCommand::new(&arg_vec);

    // We can also iterate over our collected arguments, skipping the name of the
    // program itself, and pass them as arguments to our newly created process
    tp_command.build_command();
    // Run the process
    if let Err(e) = tp_command.run() {
        eprintln!("Command Error: {e}");
        process::exit(1);
    }
}

struct TPCommand {
    command: Command,
    prog_args: Vec<String>,
}

impl TPCommand {
    fn new(arg_vec: &[String]) -> TPCommand {
        let prog_name = arg_vec[1].clone();
        let command = Command::new(&prog_name);
        let mut prog_args: Vec<String> = Vec::new();
        for (index, item) in arg_vec.iter().enumerate() {
            if index < 2 {
                continue;
            }
            prog_args.push(item.clone());
        }

        TPCommand { command, prog_args }
    }

    fn build_command(&mut self) {
        for arg in &self.prog_args {
            self.command.arg(arg);
        }
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.command.status()?;
        Ok(())
    }
}
