
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Right,
    Left,
    Inc,
    Dec,
    Out,
    In,
    JmpFwd,
    JmpBack,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    commands: Vec<Command>,
    jmptable: HashMap<usize, usize>,
    memory: Vec<u8>,
}

impl Program {
    pub fn compile(input: &str) -> Vec<Command> {
        let mut coms = Vec::new();
        for c in input.chars() {
            let command = match c {
                '>' => Command::Right,
                '<' => Command::Left,
                '+' => Command::Inc,
                '-' => Command::Dec,
                '.' => Command::Out,
                ',' => Command::In,
                '[' => Command::JmpFwd,
                ']' => Command::JmpBack,
                _ => continue,
            };
            coms.push(command);
        }

        coms
    }

    pub fn new(commands: Vec<Command>) -> Program {
        // build jump table
        let mut jmps = Vec::new();
        let mut table = HashMap::new();
        for (i, c) in commands.iter().enumerate() {
            match c {
                Command::JmpFwd => jmps.push(i),
                Command::JmpBack => {
                    let start = jmps.pop().unwrap();
                    table.insert(start, i);
                    table.insert(i, start);
                },
                _ => {},
            }
        }

        Program {
            commands: commands,
            memory: vec![0; 30000],
            jmptable: table,
        }
    }

    pub fn from_str(input: &str) -> Program {
        Self::new(Self::compile(input))
    }

    pub fn run<R: Read, W: Write>(&mut self, input: &mut R, output: &mut W) -> io::Result<()> {
        let mut input = input.bytes();

        let mut ptr = 0;
        let mut pc = 0;
        loop {
            match self.commands[pc] {
                Command::Right => ptr += 1,
                Command::Left => ptr -= 1,
                Command::Inc => self.memory[ptr] += 1,
                Command::Dec => {
                    self.memory[ptr] -= 1;
                },
                Command::Out => {
                    output.write(&[self.memory[ptr]])?;
                },
                Command::In => match input.next() {
                    Some(res) => {
                        self.memory[ptr] = res?;
                    },
                    None => {}, // EOF, do nothing for now
                },
                Command::JmpFwd => {
                    if self.memory[ptr] == 0 {
                        pc = self.jmptable[&pc];
                    }
                },
                Command::JmpBack => {
                    if self.memory[ptr] != 0 {
                        pc = self.jmptable[&pc];
                    }
                }
            }
            pc += 1;
            
            if pc >= self.commands.len() {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::empty;

    #[test]
    fn hello_world() {
        let raw = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

        let mut output = Vec::new();
        let mut compiled = Program::from_str(raw);
        compiled.run(&mut empty(), &mut output).unwrap();
        let out: String = output.iter().map(|&b| char::from(b)).collect();
        assert_eq!(out, "Hello World!\n");
    }

    #[test]
    fn hello_world_comments() {
        let raw = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let commented = r" 1 +++++ +++               Set Cell #0 to 8
 2 [
 3     >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
 4     [                   as the cell will be cleared by the loop
 5         >++             Add 4*2 to Cell #2
 6         >+++            Add 4*3 to Cell #3
 7         >+++            Add 4*3 to Cell #4
 8         >+              Add 4 to Cell #5
 9         <<<<-           Decrement the loop counter in Cell #1
10     ]                   Loop till Cell #1 is zero
11     >+                  Add 1 to Cell #2
12     >+                  Add 1 to Cell #3
13     >-                  Subtract 1 from Cell #4
14     >>+                 Add 1 to Cell #6
15     [<]                 Move back to the first zero cell you find; this will
16                         be Cell #1 which was cleared by the previous loop
17     <-                  Decrement the loop Counter in Cell #0
18 ]                       Loop till Cell #0 is zero
19 
20 The result of this is:
21 Cell No :   0   1   2   3   4   5   6
22 Contents:   0   0  72 104  88  32   8
23 Pointer :   ^
24 
25 >>.                     Cell #2 has value 72 which is 'H'
26 >---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
27 +++++ ++..+++.          Likewise for 'llo' from Cell #3
28 >>.                     Cell #5 is 32 for the space
29 <-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
30 <.                      Cell #3 was set to 'o' from the end of 'Hello'
31 +++.----- -.----- ---.  Cell #3 for 'rl' and 'd'
32 >>+.                    Add 1 to Cell #5 gives us an exclamation point
33 >++.                    And finally a newline from Cell #6";

        let mut raw_compiled = Program::from_str(raw);
        let mut commented_compiled = Program::from_str(commented);
        assert_eq!(raw_compiled, commented_compiled);

        let mut raw_output = Vec::new();
        let mut commented_output = Vec::new();

        raw_compiled.run(&mut empty(), &mut raw_output).unwrap();
        commented_compiled.run(&mut empty(), &mut commented_output).unwrap();

        let raw_out: String = raw_output.iter().map(|&b| char::from(b)).collect();
        let commented_out: String = commented_output.iter().map(|&b| char::from(b)).collect();
        assert_eq!(raw_out, commented_out);
        assert_eq!(raw_compiled, commented_compiled);
    }

    #[test]
    fn array_size_test() {
        // taken from http://www.hevanet.com/cristofd/brainfuck/tests.b
        // "Goes to cell 30000 and reports from there with a '#'"
        let raw = "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
+++++[>+++++++<<++>-]>.<<.";
        let mut prog = Program::from_str(raw);

        let mut output = Vec::new();
        prog.run(&mut empty(), &mut output).unwrap();

        assert_eq!('#' as u8, output[0]);
    }

    #[test]
    fn obscure_tests() {
        // taken from http://www.hevanet.com/cristofd/brainfuck/tests.b
        // "Tests for several obscure problems. Should output an H."
        let raw = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
"A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let mut prog = Program::from_str(raw);

        let mut output = Vec::new();
        prog.run(&mut empty(), &mut output).unwrap();

        assert_eq!('H' as u8, output[0]);
    }
}
