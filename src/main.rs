use std::process::Command;

fn main() {
    let mut echo_hello = Command::new("echo");
    echo_hello.arg("hello");
    echo_hello.status().expect("process failed to execute");
    println!();
}
