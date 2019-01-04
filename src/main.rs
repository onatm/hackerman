use hackerman_lib::assembler::Assembler;
use hackerman_lib::parser::Parser;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::process;

struct Config {
    pub input: String,
    pub output: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: hackerman input.asm output.hack");
        }
        let input = args[1].clone();
        let output = args[2].clone();

        Ok(Config { input, output })
    }
}

fn assemble(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.input)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let instructions = Parser::parse(&contents);

    let codes = Assembler::assemble(instructions);

    let mut output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(config.output)?;

    for code in codes {
        writeln!(output_file, "{:016b}", code)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("hackerman...");
    let _ = assemble(config);
}
