use nom::rest;

pub struct Parser;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Dest(&'a str),
  Comp(&'a str),
  Jump(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
  Address(&'a str),
  Compute {
    dest: Option<Token<'a>>,
    comp: Option<Token<'a>>,
    jump: Option<Token<'a>>,
  },
  Label(&'a str),
}

named!(parse_address<&str, Instruction>,
    do_parse!(
        tag!("@") >> 
        addr: rest >>
        (Instruction::Address(addr))
    )
);

named!(parse_label<&str, Instruction>,
    do_parse!(
        label: delimited!(char!('('), take_until!(")"), char!(')')) >>
        (Instruction::Label(label))
    )
);

named!(lex_dest<&str, Token>,
   do_parse!(
        dest:
        terminated!(
            alt!(
                tag!("AMD") |
                tag!("AD") |
                tag!("AM") |
                tag!("MD") |
                tag!("D") |
                tag!("A") |
                tag!("M")),
                tag!("=")
        ) >>
        (Token::Dest(dest))
   )
);

named!(lex_comp<&str, Token>,
    do_parse!(
        comp: alt_complete!(
            tag!("0") |
            tag!("1") |
            tag!("-1") |
            tag!("D+1") |
            tag!("A+1") |
            tag!("M+1") |
            tag!("D-1") |
            tag!("A-1") |
            tag!("M-1") |
            tag!("D+A") |
            tag!("D+M") |
            tag!("D-A") |
            tag!("D-M") |
            tag!("A-D") |
            tag!("M-D") |
            tag!("D&A") |
            tag!("D&M") |
            tag!("D|A") |
            tag!("D|M") |
            tag!("D") |
            tag!("A") |
            tag!("M") |
            tag!("!D") |
            tag!("!A") |
            tag!("!M") |
            tag!("-D") |
            tag!("-A") |
            tag!("-M")
        ) >>
        (Token::Comp(comp))
    )
);

named!(lex_jump<&str, Token>,
    do_parse!(
        jump: preceded!(
            tag!(";"),
            alt!(
               tag!("JGT") |
               tag!("JEQ") |
               tag!("JGE") |
               tag!("JLT") |
               tag!("JNE") |
               tag!("JLE") |
               tag!("JMP") 
            )
        ) >>
        (Token::Jump(jump))
    )
);

named!(parse_compute<&str, Instruction>,
    do_parse!(
        dest: opt!(complete!(lex_dest)) >>
        comp: opt!(complete!(lex_comp)) >>
        jump: opt!(complete!(lex_jump)) >>
        (Instruction::Compute{ dest: dest, comp: comp, jump: jump })
    )
);

named!(parse_instruction<&str, Instruction>,
    alt!(
        parse_label |
        parse_address |
        parse_compute
    )
);

impl Parser {
  pub fn parse(input: &str) -> Instruction {
    parse_instruction(input).unwrap().1
  }
}

#[test]
fn address_instruction() {
  let expected_instruction = Instruction::Address(&"sum");

  let instruction = parse_instruction("@sum");
  let (_, instruction) = instruction.unwrap();

  assert_eq!(instruction, expected_instruction);
}

#[test]
fn label_instruction() {
  let expected_instruction = Instruction::Label(&"LABEL");

  let instruction = parse_instruction("(LABEL)");
  let (_, instruction) = instruction.unwrap();

  assert_eq!(instruction, expected_instruction);
}

#[test]
fn dest_token() {
  let expected_token = Token::Dest(&"M");

  let token = lex_dest("M=");
  let (_, token) = token.unwrap();

  assert_eq!(token, expected_token);
}

#[test]
fn comp_token() {
  let expected_token = Token::Comp(&"D&A");

  let token = lex_comp("D&A");
  let (_, token) = token.unwrap();

  assert_eq!(token, expected_token);
}

#[test]
fn jump_token() {
  let expected_token = Token::Jump(&"JNE");

  let token = lex_jump(";JNE");
  let (_, token) = token.unwrap();

  assert_eq!(token, expected_token);
}

#[test]
fn full_compute_instruction() {
  let expected_instruction = Instruction::Compute {
    dest: Some(Token::Dest(&"M")),
    comp: Some(Token::Comp(&"D|A")),
    jump: Some(Token::Jump(&"JNE")),
  };

  let instruction = parse_instruction("M=D|A;JNE");
  let (_, instruction) = instruction.unwrap();

  assert_eq!(instruction, expected_instruction);
}

#[test]
fn compute_instruction_with_dest() {
  let expected_instruction = Instruction::Compute {
    dest: Some(Token::Dest(&"D")),
    comp: Some(Token::Comp(&"A")),
    jump: None,
  };

  let instruction = parse_instruction("D=A");
  let (_, instruction) = instruction.unwrap();

  assert_eq!(instruction, expected_instruction);
}
