use std::fs::read_to_string;
use std::collections::HashMap;

/*
0000 | ADD -> r<n>     | add -> r<n>
0001 | SUB -> r<n>     | sub -> r<n>
0010 | r<n> -> a0      | r<n> -> a<n>
0011 | r<n> -> a1      | 
0100 | <n> -> clk ? == | if == jump -> <n>
0101 | <n> -> clk ? != | if != jump -> <n>
0110 | <n> -> a0       | <n> -> a<n>
0111 | <n> -> a1       |
1000 | <n> -> ck       | jump -> <n>
1111 | dummy           | pass
*/

struct Program {
    instructions: Vec<String>
}

impl Program {
    fn new(path: String) -> Option<Program> {
        match read_to_string(path) {
            Ok(raw) => {
                let mut lines: Vec<String> = raw.lines().map(|x| x.to_string()).collect::<Vec<String>>();
                let mut loops: Vec<usize> = vec![];
                lines.insert(0, String::from("pass"));
                lines.insert(0, String::from("pass"));
                let mut functions: Vec<usize> = vec![];
                let mut indented: bool = false;
                for (idx, line) in lines.iter().enumerate() {
                    if line.starts_with('\t') {
                        indented = true;
                    } else if line.starts_with("loop") {
                        loops.push(idx + 1);
                        indented = true;
                    } else if line.starts_with("fn") {
                        functions.push(idx + 1);
                        indented = true;
                    } else {
                        if indented {
                            if loops.len() % 2 == 1 {
                                loops.push(idx);
                            } else if functions.len() % 2 == 1 {
                                functions.push(idx);
                            }
                        }
                        indented = false;
                    }
                }
                lines = lines.iter().map(|x|
                    if x.starts_with('\t') { x.strip_prefix('\t').unwrap().to_string() }
                    else { x.to_string() } ).collect::<Vec<String>>();
                if loops.len() % 2 == 1 {
                    loops.push(lines.len());
                } else if functions.len() % 2 == 1 {
                    functions.push(lines.len());
                }
                            
                for (i, idx) in loops.iter().enumerate() {
                    match i % 2 {
                        0 => {
                            lines[idx-1] = String::from("pass");
                        }
                        _ => {
                            lines.insert(*idx, format!("jump -> {}", loops[i-1]-2));
                        }
                    }
                }

                return Some(Program { instructions: lines });
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        None
    }

    fn compile(&self) -> Vec<String> {
        for line in &self.lines {
            
}

fn main() {
    let program = Program::new(String::from("test.mc"));
}
