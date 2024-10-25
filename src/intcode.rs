use std::collections::VecDeque;
use std::str::FromStr;

pub enum CpuStatus {
    Output(i64),
    WaitForInput,
    Finished,
    Running,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Opcode {
    ADD,
    MUL,
    IN,
    OUT,
    JIT,
    JIF,
    LT,
    EQ,
    ARB,
    BRK
}

#[derive(Debug, Eq, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Opcode {
    fn from(val: i64) -> Self {
        match val {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            3 => Opcode::IN,
            4 => Opcode::OUT,
            5 => Opcode::JIT,
            6 => Opcode::JIF,
            7 => Opcode::LT,
            8 => Opcode::EQ,
            9 => Opcode::ARB,
            99 => Opcode::BRK,
            _ => panic!("Invalid opcode: {}!", val)
        }
    }
}

impl From<i64> for ParamMode {
    fn from(val: i64) -> Self {
        match val {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Invalid parameter mode: {}!", val)
        }
    }
}

#[derive(Clone)]
pub struct Cpu {
    pub code: Vec<i64>,
    pc: usize,
    inputs: VecDeque<i64>,
    relative_base: i64,
}

fn str2code(str_code: &str) -> Vec<i64> {
    str_code
        .split(",")
        .filter_map(|c| i64::from_str(c).ok())
        .collect()
}

impl Cpu {
    pub fn new(code: &str) -> Cpu {
        return Cpu {
            code: str2code(code),
            pc: 0,
            inputs: VecDeque::new(),
            relative_base: 0,
        };
    }


    pub fn push_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

  pub fn run_until_output(&mut self) -> Option<i64> {
        loop {
            match self.step() {
                CpuStatus::Output(val) => return Some(val),
                CpuStatus::Finished => return None,
                CpuStatus::WaitForInput => panic!("Expected input!"),
                CpuStatus::Running => continue,
            }
        }
    }
    pub fn run(&mut self) -> i64 {
        let mut last_out: i64 = 0;
        loop {
            match self.step() {
                CpuStatus::Output(x) => last_out = Some(x).unwrap(),
                CpuStatus::WaitForInput => return last_out,
                CpuStatus::Finished => return last_out,
                _ => continue,
            }
        }
    }

    pub fn step(&mut self) -> CpuStatus {
        if self.pc >= self.code.len() {
            return CpuStatus::Finished;
        }

        let (opcode, param_modes) = self.decode(self.code[self.pc]);

        match opcode {
            Opcode::ADD => self.add(&param_modes),
            Opcode::MUL => self.mul(&param_modes),
            Opcode::IN => return self.input(&param_modes),
            Opcode::OUT => return self.output(&param_modes),
            Opcode::JIT => self.jit(&param_modes),
            Opcode::JIF => self.jif(&param_modes),
            Opcode::LT => self.lt(&param_modes),
            Opcode::EQ => self.eq(&param_modes),
            Opcode::ARB => self.arb(&param_modes),
            Opcode::BRK => return CpuStatus::Finished,
        }

        CpuStatus::Running
    }

    fn decode(&mut self, instruction: i64) -> (Opcode, Vec<ParamMode>) {
        let opcode = instruction % 100;
        let param_modes = vec![
            ((instruction / 100) % 10).into(),
            ((instruction / 1000) % 10).into(),
            ((instruction / 10000) % 10).into(),
        ];
        (opcode.into(), param_modes)
    }

    fn get_param(&self, idx: usize, param_modes: &Vec<ParamMode>) -> i64 {
        let param = self.code[self.pc + idx + 1];
        match param_modes[idx] {
            ParamMode::Position => self.read_mem(param as usize),
            ParamMode::Immediate => param,
            ParamMode::Relative => self.read_mem((param + self.relative_base) as usize),
        }
    }

    fn dst(&self, idx: usize, param_modes: &Vec<ParamMode>) -> usize {
        let dst = self.code[self.pc + idx + 1];
        match param_modes[idx] {
            ParamMode::Position => dst as usize,
            ParamMode::Relative => (dst + self.relative_base) as usize,
            ParamMode::Immediate => panic!("Invalid mode for dst"),
        }
    }

    fn get_two_operands(&self, param_modes: &Vec<ParamMode>) -> (i64, i64) {
        let p1 = self.get_param(0, param_modes);
        let p2 = self.get_param(1, param_modes);
        (p1, p2)
    }

    fn get_three_operands(&self, param_modes: &Vec<ParamMode>) -> (i64, i64, usize) {
        let p1 = self.get_param(0, param_modes);
        let p2 = self.get_param(1, param_modes);
        let dst = self.dst(2, param_modes);
        (p1, p2, dst as usize)
    }

    fn write_mem(&mut self, idx: usize, val: i64) {
        if idx >= self.code.len() {
            self.code.resize(idx + 1, 0);
        }
        self.code[idx] = val;
    }

    fn read_mem(&self, idx: usize) -> i64 {
        if idx >= self.code.len() {
            return 0;
        }
        self.code[idx]
    }

    fn add(&mut self, param_modes: &Vec<ParamMode>) {
        let (o1, o2, dst) = self.get_three_operands(param_modes);
        self.write_mem(dst, o1 + o2);
        self.pc += 4;
    }

    fn mul(&mut self, param_modes: &Vec<ParamMode>) {
        let (o1, o2, dst) = self.get_three_operands(param_modes);
        self.write_mem(dst, o1 * o2);
        self.pc += 4;
    }

    fn input(&mut self, param_modes: &Vec<ParamMode>) -> CpuStatus {
        let dst = self.dst(0, param_modes);
        match self.inputs.pop_front() {
            Some(val) => {
                self.write_mem(dst, val);
                self.pc += 2;
                CpuStatus::Running
            }
            None => CpuStatus::WaitForInput,
        }
    }

    fn output(&mut self, param_modes: &Vec<ParamMode>) -> CpuStatus {
        let o1 = self.get_param(0, param_modes);
        self.pc += 2;
        CpuStatus::Output(o1)
    }

    fn jit(&mut self, param_modes: &Vec<ParamMode>) {
        let (o1, o2) = self.get_two_operands(param_modes);
        self.pc = if o1 != 0 { o2 as usize } else { self.pc + 3 };
    }

    fn jif(&mut self, param_modes: &Vec<ParamMode>) {
        let (o1, o2) = self.get_two_operands(param_modes);
        self.pc = if o1 == 0 { o2 as usize } else { self.pc + 3 };
    }

    fn lt(&mut self, param_modes: &Vec<ParamMode>) {
        let (o1, o2, dst) = self.get_three_operands(param_modes);
        self.write_mem(dst, if o1 < o2 { 1 } else { 0 });
        self.pc += 4;
    }

    fn eq(&mut self, param_modes: &Vec<ParamMode>) {
        let (o1, o2, dst) = self.get_three_operands(param_modes);
        self.write_mem(dst, if o1 == o2 { 1 } else { 0 });
        self.pc += 4;
    }

    fn arb(&mut self, param_modes: &Vec<ParamMode>) {
        let o1 = self.get_param(0, param_modes);
        self.relative_base += o1;
        self.pc += 2;
    }
}
