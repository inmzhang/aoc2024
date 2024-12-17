#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut machine = parse(input);
    // for _ in 0..20 {
    //     dbg!(&machine.pointer);
    //     dbg!(&machine.output);
    //     machine.tick();
    // }
    while machine.tick() {}
    let res = machine
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    Ok(res)
}

fn parse(input: &str) -> Machine {
    let (s1, s2) = input.split_once("\n\n").expect("invalid input");
    let mut registers = [0usize; 3];
    for (i, n) in s1
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse::<usize>().unwrap())
        .take(3)
        .enumerate()
    {
        registers[i] = n;
    }
    let (_, instructions) = s2.trim().split_once(": ").expect("invalid input");
    let program: Vec<u8> = instructions
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    Machine::new(registers, program)
}

#[derive(Debug)]
struct Machine {
    registers: [usize; 3],
    program: Vec<u8>,
    pointer: usize,
    output: Vec<usize>,
}

impl Machine {
    fn new(registers: [usize; 3], program: Vec<u8>) -> Self {
        Self {
            registers,
            program,
            pointer: 0,
            output: Vec::new(),
        }
    }
    fn tick(&mut self) -> bool {
        if self.pointer >= self.program.len() {
            return false;
        }
        let opcode = self.program[self.pointer];
        let operand = self.program[self.pointer + 1];
        match opcode {
            0 => self.registers[0] /= 2usize.pow(self.combo_operand(operand) as u32),
            1 => self.registers[1] ^= operand as usize,
            2 => self.registers[1] = self.combo_operand(operand) % 8,
            3 => {
                if self.registers[0] != 0 {
                    self.pointer = operand as usize;
                    return true;
                }
            }
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push(self.combo_operand(operand) % 8),
            6 => {
                self.registers[1] =
                    self.registers[0] / 2usize.pow(self.combo_operand(operand) as u32)
            }
            7 => {
                self.registers[2] =
                    self.registers[0] / 2usize.pow(self.combo_operand(operand) as u32)
            }
            _ => panic!("invalid opcode"),
        }
        self.pointer += 2;
        true
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            op if (0..=3).contains(&op) => op as usize,
            op if (4..=6).contains(&op) => self.registers[op as usize - 4],
            _ => panic!("invalid operand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }
}
