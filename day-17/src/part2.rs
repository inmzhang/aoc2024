#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut machine = parse(input);
    let mut factors = vec![0; machine.program.len()];

    loop {
        let mut init_a = 0;
        for (i, f) in factors.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f
        }
        machine.restart(init_a as usize);
        while machine.tick() {}

        if machine.output == machine.program {
            break Ok(init_a.to_string());
        }

        for i in (0..machine.program.len()).rev() {
            if machine.output.len() < i {
                factors[i] += 1;
                break;
            }
            if machine.output[i] != machine.program[i] {
                factors[i] += 1;
                break;
            }
        }
    }
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
    start_b: usize,
    start_c: usize,
    program: Vec<u8>,
    pointer: usize,
    output: Vec<u8>,
}

impl Machine {
    fn new(registers: [usize; 3], program: Vec<u8>) -> Self {
        Self {
            registers,
            start_b: registers[1],
            start_c: registers[2],
            program,
            pointer: 0,
            output: Vec::new(),
        }
    }

    fn restart(&mut self, register_a: usize) {
        self.registers[0] = register_a;
        self.registers[1] = self.start_b;
        self.registers[2] = self.start_c;
        self.pointer = 0;
        self.output.clear();
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
            5 => {
                let out = (self.combo_operand(operand) % 8) as u8;
                self.output.push(out);
            }
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
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", process(input)?);
        Ok(())
    }
}
