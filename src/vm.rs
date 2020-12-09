#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug, Clone)]
pub struct State {
    pub ip: i32,
    pub acc: i32,
}

impl Default for State {
    fn default() -> State {
        State{
            ip: 0,
            acc: 0,
        }
    }
}

pub struct VM {
    instructions: Vec<Instruction>,
    state: State,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>, state: State) -> VM {
        VM{
            instructions,
            state,
        }
    }

    pub fn instructions(&self) -> &[Instruction] { &self.instructions }

    pub fn state(&self) -> &State { &self.state }

    fn get_instruction(&self, ip: i32) -> anyhow::Result<Instruction> {
        if ip < 0 || ip >= self.instructions.len() as i32 {
            Err(anyhow::Error::msg("ip out of range"))?;
        }

        Ok(self.instructions[ip as usize])
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        let inst = self.get_instruction(self.state.ip)?;
        self.state.ip += 1;

        match inst {
            Instruction::Acc(a) => self.state.acc += a,
            Instruction::Jmp(offset) => self.state.ip += offset - 1,
            Instruction::Nop(_) => {},
        }

        Ok(())
    }
}

