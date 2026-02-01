use std::error::Error;
use std::process::Command;

pub struct TPCommand {
    command: Command,
    prog_args: Vec<String>,
}

impl TPCommand {
    pub fn new(arg_vec: &[String]) -> TPCommand {
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

    pub fn build_command(&mut self) {
        for arg in &self.prog_args {
            self.command.arg(arg);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.command.status()?;
        Ok(())
    }
}
