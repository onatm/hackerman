use crate::parser::Instruction;
use crate::parser::Token;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Assembler;

impl Assembler {
  pub fn assemble(instructions: &[Instruction]) -> Vec<u16> {
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

    for instruction in instructions {
      match instruction {
        Instruction::Label(label) => {
          labels.insert(label, pc as u16);
        }
        _ => pc += 1,
      };
    }

    for instruction in instructions {
      let code = match instruction {
        Instruction::Compute { dest, comp, jump } => {
          translate_compute_instruction(dest, comp, jump)
        }
        Instruction::Address(symbol) => {
          let code: u16;

          if let Result::Ok(symbol) = i32::from_str(symbol) {
            code = symbol as u16;
          } else if symbol_table.contains_key(symbol) {
            code = symbol_table[symbol];
          } else if labels.contains_key(symbol) {
            code = labels[symbol];
          } else {
            let address = symbol_table.len() - 7;

            if address >= 16384 {
              panic!("Out of memory! Too many user defined symbols!");
            }

            code = address as u16;

            symbol_table.insert(symbol, code);
          }

          code
        }
        _ => continue,
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
    "0" => 0b010_1010,
    "1" => 0b011_1111,
    "-1" => 0b011_1010,
    "D" => 0b000_1100,
    "A" => 0b011_0000,
    "!D" => 0b000_1101,
    "!A" => 0b011_0001,
    "-D" => 0b000_1111,
    "-A" => 0b011_0011,
    "D+1" => 0b001_1111,
    "A+1" => 0b011_0111,
    "D-1" => 0b000_1110,
    "A-1" => 0b011_0010,
    "D+A" => 0b000_0010,
    "D-A" => 0b001_0011,
    "A-D" => 0b000_0111,
    "D&A" => 0b000_0000,
    "D|A" => 0b001_0101,
    "M" => 0b111_0000,
    "!M" => 0b111_0001,
    "-M" => 0b111_0011,
    "M+1" => 0b111_0111,
    "M-1" => 0b111_0010,
    "D+M" => 0b100_0010,
    "D-M" => 0b101_0011,
    "M-D" => 0b100_0111,
    "D&M" => 0b100_0000,
    "D|M" => 0b101_0101,
    comp => panic!("Invalid comp instruction: {}", comp),
  };

  code << 6
}

fn translate_jump(jump: &str) -> u16 {
  match jump {
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

#[test]
fn full_compute_instruction() {
  let expected_binary: u16 = 0b1111_1100_1001_1111;

  let binary = translate_compute_instruction(
    &Some(Token::Dest(&"MD")),
    &Token::Comp(&"M-1"),
    &Some(Token::Jump(&"JMP")),
  );

  assert_eq!(binary, expected_binary);
}

#[test]
fn compute_instruction_with_dest_and_comp() {
  let expected_binary: u16 = 0b1110_0100_1101_0000;

  let binary = translate_compute_instruction(&Some(Token::Dest(&"D")), &Token::Comp(&"D-A"), &None);

  assert_eq!(binary, expected_binary);
}

#[test]
fn compute_instruction_with_comp_and_jump() {
  let expected_binary: u16 = 0b1110_1010_1000_0101;

  let binary = translate_compute_instruction(&None, &Token::Comp(&"0"), &Some(Token::Jump(&"JNE")));

  assert_eq!(binary, expected_binary);
}

#[test]
fn assemble() {
  let expected_binaries: Vec<u16> = vec![
    0b0000_0001_0000_0000,
    0b1110_1100_0001_0000,
    0b0000_0000_0000_0000,
    0b1110_0011_0000_1000,
    0b0000_0000_1000_0101,
    0b1110_1010_1000_0111,
    0b0000_0000_0000_1111,
    0b1110_0011_0000_1000,
    0b0000_0000_0000_0000,
    0b1111_1100_1010_1000,
    0b1111_1100_0001_0000,
    0b1110_1100_1010_0000,
    0b1111_0001_1101_0000,
    0b1110_1010_1000_1000,
    0b0000_0000_0001_0011,
    0b1110_0011_0000_0101,
    0b0000_0000_0000_0000,
    0b1111_1100_1010_0000,
    0b1110_1110_1000_1000,
  ];

  let instructions: &[Instruction] = &[
    Instruction::Address(&"256"),
    Instruction::Compute {
      dest: Some(Token::Dest(&"D")),
      comp: Token::Comp(&"A"),
      jump: None,
    },
    Instruction::Address(&"SP"),
    Instruction::Compute {
      dest: Some(Token::Dest(&"M")),
      comp: Token::Comp(&"D"),
      jump: None,
    },
    Instruction::Address(&"133"),
    Instruction::Compute {
      dest: None,
      comp: Token::Comp(&"0"),
      jump: Some(Token::Jump(&"JMP")),
    },
    Instruction::Address(&"R15"),
    Instruction::Compute {
      dest: Some(Token::Dest(&"M")),
      comp: Token::Comp(&"D"),
      jump: None,
    },
    Instruction::Address(&"SP"),
    Instruction::Compute {
      dest: Some(Token::Dest(&"AM")),
      comp: Token::Comp(&"M-1"),
      jump: None,
    },
    Instruction::Compute {
      dest: Some(Token::Dest(&"D")),
      comp: Token::Comp(&"M"),
      jump: None,
    },
    Instruction::Compute {
      dest: Some(Token::Dest(&"A")),
      comp: Token::Comp(&"A-1"),
      jump: None,
    },
    Instruction::Compute {
      dest: Some(Token::Dest(&"D")),
      comp: Token::Comp(&"M-D"),
      jump: None,
    },
    Instruction::Compute {
      dest: Some(Token::Dest(&"M")),
      comp: Token::Comp(&"0"),
      jump: None,
    },
    Instruction::Address(&"END_EQ"),
    Instruction::Compute {
      dest: None,
      comp: Token::Comp(&"D"),
      jump: Some(Token::Jump(&"JNE")),
    },
    Instruction::Address(&"SP"),
    Instruction::Compute {
      dest: Some(Token::Dest(&"A")),
      comp: Token::Comp(&"M-1"),
      jump: None,
    },
    Instruction::Compute {
      dest: Some(Token::Dest(&"M")),
      comp: Token::Comp(&"-1"),
      jump: None,
    },
    Instruction::Label(&"END_EQ"),
  ];

  let binaries = Assembler::assemble(instructions);

  assert_eq!(binaries, expected_binaries);
}
