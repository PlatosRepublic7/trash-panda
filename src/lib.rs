use std::fs::DirBuilder;
use std::io::Result;
use std::process::Command;
use std::process::ExitStatus;
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CToken {
    ProgName(String),
    ProgFlag(String),
    Path(String),
}

enum CommandType {
    Remove,
    Move,
    Copy,
}

pub struct Tokenizer {
    arg_vec: Vec<String>,
    token_vec: Vec<CToken>,
}

impl Tokenizer {
    pub fn new(in_arg_vec: &[String]) -> Tokenizer {
        let mut arg_vec: Vec<String> = Vec::new();
        let token_vec: Vec<CToken> = Vec::new();

        for arg in in_arg_vec {
            arg_vec.push(arg.clone());
        }

        Tokenizer { arg_vec, token_vec }
    }

    pub fn gen_tokens(&mut self) {
        for (index, item) in self.arg_vec.iter().enumerate() {
            if index == 0 {
                continue;
            } else if index == 1 {
                self.token_vec.push(CToken::ProgName(item.clone()));
            } else {
                let c_token = self.get_token_type(item);
                self.token_vec.push(c_token);
            }
        }
    }

    pub fn get_tokens(&self) -> Vec<CToken> {
        self.token_vec.clone()
    }

    fn get_token_type(&self, arg_string: &str) -> CToken {
        for char in arg_string.chars() {
            if char == '-' {
                let t_str = String::from(arg_string);
                return CToken::ProgFlag(t_str);
            }
        }
        let t_str = String::from(arg_string);
        CToken::Path(t_str)
    }
}

pub struct TPCommand {
    token_vec: Vec<CToken>,
    trash_dir: String,
}

impl TPCommand {
    pub fn new(in_token_vec: &Vec<CToken>, home_dir: &str) -> TPCommand {
        let mut token_vec: Vec<CToken> = Vec::new();
        let mut trash_dir: String = String::from(home_dir);
        trash_dir += "/.local/share/test_trash";
        make_trash_dir(&trash_dir);

        for token in in_token_vec {
            token_vec.push(token.clone());
        }

        TPCommand {
            token_vec,
            trash_dir,
        }
    }

    pub fn run(&self) -> Result<ExitStatus> {
        let mut command = self.make_command();
        let status = command.status()?;
        Ok(status)
    }

    fn make_command(&self) -> Command {
        let mut c_prog_name = String::new();
        let mut c_prog_flags: Vec<String> = Vec::new();
        let mut c_path = String::new();

        for token in &self.token_vec {
            match token {
                CToken::ProgName(prog_name) => {
                    c_prog_name = prog_name.clone();
                }
                CToken::ProgFlag(prog_flag) => {
                    c_prog_flags.push(prog_flag.clone());
                }
                CToken::Path(path) => {
                    c_path = path.clone();
                }
            }
        }
        let mut command = Command::new(c_prog_name);
        for flag in &c_prog_flags {
            command.arg(flag.clone());
        }
        command.arg(c_path);
        command
    }
}

fn make_trash_dir(trash_dir: &str) {
    DirBuilder::new().recursive(true).create(trash_dir).unwrap();
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn generate_token_vec() {
        let mut args: Vec<String> = Vec::new();
        let prog_name = String::from("rm");
        let prog_flag = String::from("-rf");
        let prog_directory = String::from("/some/directory");
        args.push(String::from("tp"));
        args.push(prog_name.clone());
        args.push(prog_flag.clone());
        args.push(prog_directory.clone());
        let mut tokenizer = Tokenizer::new(&args);
        tokenizer.gen_tokens();
        let token_vec = tokenizer.get_tokens();
        let correct_token_vec = vec![
            CToken::ProgName(prog_name),
            CToken::ProgFlag(prog_flag),
            CToken::Path(prog_directory),
        ];
        assert_eq![correct_token_vec, token_vec];
    }
}
