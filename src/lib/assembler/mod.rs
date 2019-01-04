use crate::parser::Instruction;
use crate::parser::Token;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Assembler;

impl Assembler {
  pub fn assemble(instructions: Vec<Instruction>) -> Vec<u16> {
    let mut codes: Vec<u16> = Vec::new();

    let mut symbol_table: HashMap<&str, u16> = HashMap::new();

    symbol_table.insert("SP", 0x0);
    symbol_table.insert("LCL", 0x1);
    symbol_table.insert("ARG", 0x2);
    symbol_table.insert("THIS", 0x3);
    symbol_table.insert("THAT", 0x4);
    symbol_table.insert("R0", 0x0);
    symbol_table.insert("R1", 0x1);
    symbol_table.insert("R2", 0x2);
    symbol_table.insert("R3", 0x3);
    symbol_table.insert("R4", 0x4);
    symbol_table.insert("R5", 0x5);
    symbol_table.insert("R6", 0x6);
    symbol_table.insert("R7", 0x7);
    symbol_table.insert("R8", 0x8);
    symbol_table.insert("R9", 0x9);
    symbol_table.insert("R10", 0xa);
    symbol_table.insert("R11", 0xb);
    symbol_table.insert("R12", 0xc);
    symbol_table.insert("R13", 0xd);
    symbol_table.insert("R14", 0xe);
    symbol_table.insert("R15", 0xf);
    symbol_table.insert("SCREEN", 0x4000);
    symbol_table.insert("KBD", 0x6000);

    let mut pc = 0;

    let mut labels: HashMap<&str, u16> = HashMap::new();

    for instruction in instructions.iter() {
      match instruction {
        Instruction::Label(label) => {
          labels.insert(label, pc as u16);
        }
        _ => pc += 1,
      };
    }

    for instruction in instructions.iter() {
      let code = match instruction {
        Instruction::Compute { dest, comp, jump } => {
          translate_compute_instruction(dest, comp, jump)
        }
        Instruction::Address(symbol) => {
          let code: u16;

          if let Result::Ok(address) = i32::from_str(symbol) {
            code = address as u16;
          } else {
            if symbol_table.contains_key(symbol) {
              code = *symbol_table.get(symbol).unwrap();
            } else if labels.contains_key(symbol) {
              code = *labels.get(symbol).unwrap();
            } else {
              let address = symbol_table.len() - 7;

              if address >= 16384 {
                panic!("Out of memory! Too many user defined symbols!");
              }

              code = address as u16;

              symbol_table.insert(symbol, code);
            }
          }

          code
        }
        Instruction::Label(..) => {
          continue;
        }
      };

      codes.push(code);
    }

    codes
  }
}

fn translate_compute_instruction(dest: &Option<Token>, comp: &Token, jump: &Option<Token>) -> u16 {
  let dest = match dest {
    Some(Token::Dest(dest)) => translate_dest(dest),
    None => 0b000,
    _ => panic!("Invalid token type in dest!"),
  };

  let comp = match comp {
    Token::Comp(comp) => translate_comp(comp),
    _ => panic!("Invalid token type in comp!"),
  };

  let jump = match jump {
    Some(Token::Jump(jump)) => translate_jump(jump),
    None => 0b000,
    _ => panic!("Invalid token type in jump!"),
  };

  (0b111 << 13) | dest | comp | jump
}

fn translate_dest(dest: &str) -> u16 {
  let code = match dest {
    "" => 0b000,
    "M" => 0b001,
    "D" => 0b010,
    "MD" => 0b011,
    "A" => 0b100,
    "AM" => 0b101,
    "AD" => 0b110,
    "ADM" => 0b111,
    dest => panic!("Invalid dest instruction: {}", dest),
  };

  code << 3
}

fn translate_comp(comp: &str) -> u16 {
  let code = match comp {
    "0" => 0b0_101010,
    "1" => 0b0_111111,
    "-1" => 0b0_111010,
    "D" => 0b0_001100,
    "A" => 0b0_110000,
    "!D" => 0b0_001101,
    "!A" => 0b0_110001,
    "-D" => 0b0_001111,
    "-A" => 0b0_110011,
    "D+1" => 0b0_011111,
    "A+1" => 0b0_110111,
    "D-1" => 0b0_001110,
    "A-1" => 0b0_110010,
    "D+A" => 0b0_000010,
    "D-A" => 0b0_010011,
    "A-D" => 0b0_000111,
    "D&A" => 0b0_000000,
    "D|A" => 0b0_010101,
    "M" => 0b1_110000,
    "!M" => 0b1_110001,
    "-M" => 0b1_110011,
    "M+1" => 0b1_110111,
    "M-1" => 0b1_110010,
    "D+M" => 0b1_000010,
    "D-M" => 0b1_010011,
    "M-D" => 0b1_000111,
    "D&M" => 0b1_000000,
    "D|M" => 0b1_010101,
    comp => panic!("Invalid comp instruction: {}", comp),
  };

  code << 6
}

fn translate_jump(jump: &str) -> u16 {
  match jump {
    "" => 0b000,
    "JGT" => 0b001,
    "JEQ" => 0b010,
    "JGE" => 0b011,
    "JLT" => 0b100,
    "JNE" => 0b101,
    "JLE" => 0b110,
    "JMP" => 0b111,
    jump => panic!("Invalid jump instruction: {}", jump),
  }
}
