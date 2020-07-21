use lexer::Lexer;
use std::io::{self, Write};
use std::process;

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        match io::stdin().read_line(&mut cmd) {
            Ok(_n) => {
                let lexer = Lexer::new(cmd);

                for i in lexer.into_iter() {
                    match i.literal.as_str() {
                        "exit" => process::exit(0),
                        _ => println!("{:?}", i),
                    };
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
