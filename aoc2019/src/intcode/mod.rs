//! Module for a re-usable Intcode interpreter & assorted helper types

/// Valid operations the Intcode interpreter can perform
#[derive(Debug, PartialEq)]
enum Opcode {
    /// 1  - Add together numbers read from the positions specified by `ins + 1` and `ins + 2`,
    ///      then write the result to the position specified by `ins + 3`, then increment the
    ///      instruction pointer.
    Add,

    /// 2  - Multiply together numbers read from the positions specified by `ins + 1` and `ins +
    ///      2`, then write the result to the position specified by `ins + 3`, then increment the
    ///      instruction pointer.
    Mul,

    /// 99 - Immediately halt program execution
    Halt,
}

impl TryFrom<u32> for Opcode {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Halt),
            _ => Err(format!("Unknown opcode: {value}")),
        }
    }
}

/// A re-usable Intcode interpreter to use across several days' puzzles
#[derive(Debug)]
pub struct Interpreter {
    /// The underlying tape of instructions, arguments, etc.
    tape: Vec<u32>,

    /// The instruction pointer for the interpreter to use when processing the tape
    ins_ptr: usize,
}

impl Interpreter {
    pub fn new(tape: Vec<u32>) -> Self {
        Self { tape, ins_ptr: 0 }
    }

    /// Run the interpreter until the program halts
    pub fn run_to_halt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut opcode = self.next()?;

        while opcode != Opcode::Halt {
            opcode = self.next()?;
        }

        Ok(())
    }

    /// Run the next instruction on the tape
    fn next(&mut self) -> Result<Opcode, Box<dyn std::error::Error>> {
        let opcode = Opcode::try_from(self[self.ins_ptr])?;

        match Opcode::try_from(self[self.ins_ptr])? {
            Opcode::Add => self.add(),
            Opcode::Mul => self.mul(),
            Opcode::Halt => Ok(()),
        }?;

        Ok(opcode)
    }

    /// Perform the [`Add`](Opcode::Add) operation
    fn add(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let l = self.ins_ptr + 1;
        let l = *self
            .tape
            .get(l)
            .ok_or_else(|| format!("Invalid addr: {}", l))?;
        let l = self
            .tape
            .get(l as usize)
            .ok_or_else(|| format!("Invalid addr: {l}"))?;

        let r = self.ins_ptr + 2;
        let r = *self
            .tape
            .get(r)
            .ok_or_else(|| format!("Invalid addr: {}", r))?;
        let r = self
            .tape
            .get(r as usize)
            .ok_or_else(|| format!("Invalid addr: {r}"))?;

        let res = l + r;

        let ret = self.ins_ptr + 3;
        let ret = *self
            .tape
            .get(ret)
            .ok_or_else(|| format!("Invalid addr: {}", ret))?;

        self[ret as usize] = res;
        self.ins_ptr += 4; // Need to step past the arguments

        Ok(())
    }

    /// Perform the [`Mul`](Opcode::Mul) operation
    fn mul(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let l = self.ins_ptr + 1;
        let l = *self
            .tape
            .get(l)
            .ok_or_else(|| format!("Invalid addr: {}", l))?;
        let l = self
            .tape
            .get(l as usize)
            .ok_or_else(|| format!("Invalid addr: {l}"))?;

        let r = self.ins_ptr + 2;
        let r = *self
            .tape
            .get(r)
            .ok_or_else(|| format!("Invalid addr: {}", r))?;
        let r = self
            .tape
            .get(r as usize)
            .ok_or_else(|| format!("Invalid addr: {r}"))?;

        let res = l * r;

        let ret = self.ins_ptr + 3;
        let ret = *self
            .tape
            .get(ret)
            .ok_or_else(|| format!("Invalid addr: {}", ret))?;

        self[ret as usize] = res;
        self.ins_ptr += 4; // Need to step past the arguments

        Ok(())
    }
}

impl std::ops::Index<usize> for Interpreter {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        self.tape.index(index)
    }
}

impl std::ops::IndexMut<usize> for Interpreter {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.tape.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases described in day 02, these relate to the Add, Mul, and Halt Opcodes.
    #[test]
    fn d02() {
        let cases = vec![
            // 1. Add (9, 10, 3)
            //    l: 30, r: 40, l + r: 70
            //    ret: 3
            //    tape: 1, 9, 10, 70, ...
            // 2. Mul (3, 11, 0)
            //    l: 70, r: 50, l * r: 3500
            //    ret: 0
            //    tape: 3500, 9, 10, 70, ...
            // 3. Halt
            (
                "1,9,10,3,2,3,11,0,99,30,40,50",
                "3500,9,10,70,2,3,11,0,99,30,40,50",
            ),
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ];

        for (input, expected) in cases {
            let start_tape = input
                .split(",")
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let expected_end_tape = expected
                .split(",")
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let mut program = Interpreter::new(start_tape);
            program.run_to_halt().unwrap();
            let actual_end_tape = program.tape;

            assert_eq!(actual_end_tape, expected_end_tape, "Actual != Expected");
        }
    }
}
