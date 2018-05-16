mod program;

fn main() {
    let mut output = Vec::new();
    let mut prog = program::Program::from_str("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    match prog.run(&mut std::io::empty(), &mut output) {
        Ok(_) => print!("{}", output.iter()
                        .map(|&b| char::from(b))
                        .collect::<String>()),
        Err(e) => print!("{:?}", e),
    }
}

