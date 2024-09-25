use std::fs::{read_to_string, File};
use std::io::Write;
use std::env::args;

/*
 BIN    SEMANTIC          SYNTAX
|-------------------------------------------|
|0000 | ADD -> r<n>     | add -> r<n>       |
|0001 | SUB -> r<n>     | sub -> r<n>       |
|0010 | r<n> -> a0      | r<n> -> a<n>      |
|0011 | r<n> -> a1      |                   |
|0100 | <n> -> clk ? == | if == jump -> <n> |
|0101 | <n> -> clk ? != | if != jump -> <n> |
|0110 | <n> -> a0       | <n> -> a<n>       |
|0111 | <n> -> a1       |                   |
|1000 | <n> -> ck       | jump -> <n>       |
|1001 | r<n> -> i1      | r<n> -> io1       |
|1111 | dummy           | pass              |
|-------------------------------------------|
*/

struct Program {
    instructions: Vec<String>
}

#[derive(PartialEq)]
enum Location {
    RAM,
    ALU,
    IO
}

fn usize_to_binary(mut num: i32) -> String {
    if num == 0 { return String::from("0000"); }
    let order: i32 = (num as f32).log2().floor() as i32;
    num -= (2 as i32).pow(order as u32);
    let mut binary_digits: String = String::from("1");
    for o in (0..order).rev() {
        if (2 as i32).pow(o as u32) <= num {
            num -= (2 as i32).pow(o as u32) as i32;
            binary_digits.push('1');
        } else {
            binary_digits.push('0');
        }
    }
    format!("{:0>4}", binary_digits)
}

fn decode_location(encoded: &String) -> Result<(Location, String), String> {
    if encoded.len() != 2 {
        return Err(format!("Malformed location: {encoded}"));
    }
    let numeric: usize = (encoded.chars().nth(1).unwrap() as usize) - b'0' as usize;
    let binary: String = usize_to_binary(numeric as i32);
    match encoded.chars().nth(0).unwrap() {
        'r' => {
            if numeric < 4 {
                return Ok((Location::RAM, binary));
            } else {
                return Err(format!("No such RAM address: {numeric}"));
            }
        }
        'a' => {
            if numeric < 2 {
                return Ok((Location::ALU, binary));
            } else {
                return Err(format!("No such ALU address: {numeric}"));
            }
        }
        'i' => {
            if numeric < 1 {
                return Ok((Location::IO, binary));
            } else {
                return Err(format!("No such IO address: {numeric}"));
            }
        }
        e => {
            return Err(format!("No such memory location: {e}"));
        }
    }
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
    fn compile(&self) -> Result<Vec<String>, Vec<String>> {
        let mut binary: Vec<String> = vec![];
        let mut errors: Vec<String> = vec![];
        let mut line_num: usize = 0;
        for line in self.instructions.iter().map(|x| x.to_lowercase().split(' ').map(|y| y.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>() {
            line_num += 1;
            if line.len() > 0 {
                let command = match line[0].as_str() {
                    "pass" => {
                        String::from("11111111")
                    }
                    "add" => {
                        if line[1] == "->" {
                            match decode_location(&line[2]) {
                                Ok((loc, addr)) => {
                                    match loc  {
                                        Location::RAM => {
                                            format!("0000{addr}")
                                        }
                                        Location::ALU => {
                                            errors.push(format!("Cannot add directly into ALU on line {line_num}"));
                                            String::new()
                                        }
                                        Location::IO => {
                                            errors.push(format!("Cannot add directly to IO on line {line_num}"));
                                            String::new()
                                        }
                                    }
                                }
                                Err(e) => {
                                    errors.push(e);
                                    String::new()
                                }
                            }
                        } else {
                            errors.push(format!("Expected assignment (->) on line {line_num}"));
                            String::new()
                        }
                    }
                    "sub" => {
                        if line[1] == "->" {
                            match decode_location(&line[2]) {
                                Ok((loc, addr)) => {
                                    match loc  {
                                        Location::RAM => {
                                            format!("0001{addr}")
                                        }
                                        Location::ALU => {
                                            errors.push(format!("Cannot add directly into ALU on line {line_num}"));
                                            String::new()
                                        }
                                        Location::IO => {
                                            errors.push(format!("Cannot add directly to IO on line {line_num}"));
                                            String::new()
                                        }
                                    }
                                }
                                Err(e) => {
                                    errors.push(e);
                                    String::new()
                                }
                            }
                        } else {
                            errors.push(format!("Expected assignment (->) on line {line_num}"));
                            String::new()
                        }
                    }
                    "if" => {
                        match line[1].as_str() {
                            "==" => {
                                if line[2] == "jump" {
                                    let addr: String = usize_to_binary(line[4].parse::<i32>().unwrap());
                                    format!("0100{addr}")
                                } else {
                                    errors.push(format!("IF operator only supports jump on line {line_num}"));
                                    String::new()
                                }
                            }
                            "!=" => {
                                if line[2] == "jump" {
                                    let addr: String = usize_to_binary(line[4].parse::<i32>().unwrap());
                                    format!("0101{addr}")
                                } else {
                                    errors.push(format!("IF operator only supports jump on line {line_num}"));
                                    String::new()
                                }
                            }
                            &_ => {
                                errors.push(format!("Invalid comparison: Expected == or != on line {line_num}"));
                                String::new()
                            }
                        }
                    }
                    "jump" => {
                        if line[1] == "->" {
                            match line[2].parse::<i32>() {
                                Ok(num) => {
                                    if num >= 16 {
                                        String::new()
                                    } else {
                                        let addr: String = usize_to_binary(num);
                                        format!("1000{addr}")
                                    }
                                }
                                Err(_) => {
                                    errors.push(format!("Invalid memory address on line {line_num}"));
                                    String::new()
                                }
                            }
                        } else {
                            errors.push(format!("Expected assignment (->) on line {line_num}"));
                            String::new()
                        }
                    }
                    a => {
                        match a.starts_with('r') {
                            true => {
                                match decode_location(&line[2]) {
                                    Ok((loc, addr)) => {
                                        if loc == Location::ALU {
                                            let code = match addr.as_str() {
                                                "0000" => {
                                                    String::from("0010")
                                                }
                                                "0001" => {
                                                    String::from("0011")
                                                }
                                                _ => {
                                                    errors.push(format!("No such ALU address on line {line_num}"));
                                                    String::new()
                                                }
                                            };
                                            match decode_location(&line[0]) {
                                                Ok((_, ad)) => {
                                                    format!("{code}{ad}")
                                                }
                                                Err(_) => {
                                                    errors.push(format!("No such address on line {line_num}"));
                                                    String::new()
                                                }
                                            }
                                        }
                                        else if loc == Location::ALU {
                                            errors.push(format!("Cannot write ram to ram directly on line {line_num}"));
                                            String::new()
                                        }
                                        else {
                                            let code = match addr.as_str() {
                                                "0000" => {
                                                    String::from("1001")
                                                }
                                                _ => {
                                                    errors.push(format!("No such IO address on line {line_num}"));
                                                    String::new()
                                                }
                                            };
                                            match decode_location(&line[0]) {
                                                Ok((_, ad)) => {
                                                    format!("{code}{ad}")
                                                }
                                                Err(_) => {
                                                    errors.push(format!("No such address on line {line_num}"));
                                                    String::new()
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        errors.push(format!("No such address on line {line_num}"));
                                        String::new()
                                    }
                                }
                            }
                            false => {
                                match a.parse::<i32>() {
                                    Ok(num) => {
                                        if num >= 16 {
                                            errors.push(format!("Memory bus not large enough for > 4 bit assignment on line {line_num}"));
                                            String::new()
                                        } else {
                                            let bin: String = usize_to_binary(num);
                                            match decode_location(&line[2]) {
                                                Ok((loc, addr)) => {
                                                    if loc == Location::ALU {
                                                        let code: String = match addr.as_str() {
                                                            "0000" => {
                                                                String::from("0110")
                                                            }
                                                            "0001" => {
                                                                String::from("0111")
                                                            }
                                                            _ => {
                                                                errors.push(format!("Invalid ALU address on line {line_num}"));
                                                                String::new()
                                                            }
                                                        };
                                                        format!("{code}{bin}")
                                                    } else {
                                                        errors.push(format!("Cannot write to ram, use ALU buffer on line {line_num}"));
                                                        String::new()
                                                    }
                                                }
                                                Err(e) => {
                                                    errors.push(e);
                                                    String::new()
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        errors.push(format!("Invalid number on line {line_num}"));
                                        String::new()
                                    }
                                }
                            }
                        }
                    }
                };
                binary.push(command);
            }
        }
        if errors.len() > 0 {
            return Err(errors);
        } else {
            Ok(binary)
        }
    }
}

fn main() {
    let mut args = args();
    let filename: String = args.nth(1).unwrap();
    if let Some(program) = Program::new(String::from(&filename)) {
        match program.compile() {
            Ok(binary) => {
                let mut file = File::create(filename.clone() + "exe").unwrap();
                for instruction in binary {
                    let _ = writeln!(file, "{instruction}");
                }
                println!("Success! Compiled {filename}.");
            }
            Err(errors) => {
                for error in errors {
                    println!("ERR: {error}");
                }
            }
        }
        
    } else {
        eprintln!("Could not access file.");
    }
}
