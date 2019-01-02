#[macro_use]
extern crate nom;

use nom::rest;
use std::str;

#[derive(Debug)]
enum Instruction<'a> {
    Address(&'a str),
    Compute {
        dest: Option<Token<'a>>,
        comp: Option<Token<'a>>,
        jump: Option<Token<'a>>,
    },
    Label(&'a str),
}

#[derive(Debug)]
enum Token<'a> {
    Dest(&'a str),
    Comp(&'a str),
    Jump(&'a str),
}

named!(lex_address<&str, Instruction>,
    do_parse!(
        tag!("@") >> 
        addr: rest >>
        (Instruction::Address(addr))
    )
);

named!(lex_label<&str, Instruction>,
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
        comp: alt!(
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

named!(lex_compute<&str, Instruction>,
    do_parse!(
        dest: opt!(lex_dest) >>
        comp: opt!(lex_comp) >>
        jump: opt!(lex_jump) >>
        (Instruction::Compute{ dest: dest, comp: comp, jump: jump })
    )
);

named!(lex_instruction<&str, Instruction>,
    alt!(
        lex_label |
        lex_address |
        lex_compute
    )
);

named!(lex_instructions<&str, Vec<Instruction>>, ws!(many0!(lex_instruction)));

fn main() {
    let instruction = lex_instruction("@sum");
    let (_, instruction) = instruction.unwrap_or_else(|err| panic!("{:?}", err));
    println!("{:?}", instruction);

    let instruction = lex_instruction("(LABEL)");
    let (_, instruction) = instruction.unwrap();
    println!("{:?}", instruction);

    let dest = lex_dest("D=");
    let (_, dest) = dest.unwrap();
    println!("{:?}", dest);

    let jump = lex_jump(";JLE");
    let (_, jump) = jump.unwrap();
    println!("{:?}", jump);

    let comp = lex_comp("D&A");
    let (_, comp) = comp.unwrap();
    println!("{:?}", comp);

    let compute = lex_compute("AMD=M+1;JMP");
    let (_, compute) = compute.unwrap();
    println!("{:?}", compute);

    let instruction = lex_instruction("M=D|A;JNE");
    let (_, instruction) = instruction.unwrap();
    println!("{:?}", instruction);
}
