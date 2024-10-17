use std::io::{self, Write};
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Token {
    Pipe,        // Loop delimiter
    Pound,       // Reset the tape and pointer
    Hyphen,      // Memory manipulation or pointer movement (Increment)
    Exclamation, // Memory manipulation or pointer movement (Decrement)
    Slash,       // Input
    Backslash,   // Output
    Newline,     // Newline
}

fn tokenize(input: &str) -> Vec<Token> {
    input.chars().filter_map(|c| match c {
        '|' => Some(Token::Pipe),
        '#' => Some(Token::Pound),
        '-' => Some(Token::Hyphen),
        '!' => Some(Token::Exclamation),
        '/' => Some(Token::Slash),
        '\\' => Some(Token::Backslash),
        '\n' => Some(Token::Newline),
        _ => None,  // Ignore other characters
    }).collect()
}

struct PipeInterpreter {
    tape: Vec<i32>,
    pointer: usize,
}

impl PipeInterpreter {
    fn new() -> Self {
        Self {
            tape: vec![0; 30000],  // Initialize the tape with 30,000 cells
            pointer: 0,
        }
    }

    fn interpret(&mut self, tokens: Vec<Token>) {
        let mut instruction_pointer = 0;
        let mut loop_stack = Vec::new();

        while instruction_pointer < tokens.len() {
            match tokens[instruction_pointer] {
                Token::Pipe => {
                    if self.tape[self.pointer] == 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            instruction_pointer += 1;
                            if instruction_pointer >= tokens.len() {
                                panic!("Unmatched loop start '|' at position {}", instruction_pointer);
                            }
                            match tokens[instruction_pointer] {
                                Token::Pipe => depth += 1,
                                _ => depth -= 1,
                            }
                        }
                    } else {
                        loop_stack.push(instruction_pointer);
                    }
                }
                Token::Pound => {
                    self.tape = vec![0; 30000];
                    self.pointer = 0;
                }
                Token::Hyphen => {
                    self.tape[self.pointer] = (self.tape[self.pointer] + 1) % 30000;
                }
                Token::Exclamation => {
                    self.tape[self.pointer] = (self.tape[self.pointer] + 29999) % 30000;
                }
                Token::Slash => {
                    let mut input = String::new();
                    print!("Input: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                    if let Ok(value) = input.trim().parse::<i32>() {
                        self.tape[self.pointer] = value;
                    } else {
                        println!("Invalid input, storing 0 in the cell.");
                        self.tape[self.pointer] = 0;
                    }
                }
                Token::Backslash => {
                    if self.tape[self.pointer] >= 0 && self.tape[self.pointer] <= 127 {
                        if let Some(character) = std::char::from_u32(self.tape[self.pointer] as u32) {
                            print!("{}", character);
                        } else {
                            print!("?");
                        }
                    } else {
                        print!("?");
                    }
                }
                Token::Newline => {
                    println!();
                }
            }

            instruction_pointer += 1;

            // Handle loop end
            if instruction_pointer < tokens.len() && tokens[instruction_pointer] == Token::Pipe {
                if self.tape[self.pointer] != 0 {
                    if let Some(loop_start) = loop_stack.pop() {
                        instruction_pointer = loop_start;
                    } else {
                        panic!("Unmatched loop end '|' at position {}", instruction_pointer);
                    }
                } else {
                    loop_stack.pop();
                }
            }
        }
    }
}

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.pipe>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let code = read_to_string(filename).expect("Failed to read .pipe file");
    let tokens = tokenize(&code);
    let mut interpreter = PipeInterpreter::new();
    interpreter.interpret(tokens);
}