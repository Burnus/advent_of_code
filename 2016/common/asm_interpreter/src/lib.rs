pub mod assembly_interpreter {
    #[derive(PartialEq, Eq)]
    enum State { Running, Halt, Output(isize) }

    #[derive(Debug)]
    enum Instruction {
        CpyInt(isize, usize),
        CpyReg(usize, usize),
        Inc(usize),
        Dec(usize),
        Jnz(usize, isize),
        JnzReg(usize, usize),
        Jmp(isize),
        JmpReg(usize, isize),
        Tgl(usize),
        Out(usize),
    }

    impl Instruction {
        fn parse(line: &str) -> Self {
            let components: Vec<_> = line.split(' ').collect();
            assert!((2..=3).contains(&components.len()));
            match components[0] {
                "cpy" => {
                    if let Ok(i) = components[1].parse::<isize>() {
                        Self::CpyInt(i, (components[2].bytes().next().unwrap() - b'a') as usize)
                    } else {
                        Self::CpyReg((components[1].bytes().next().unwrap() - b'a') as usize, (components[2].bytes().next().unwrap() - b'a') as usize)
                    }
                },
                "inc" => Self::Inc((components[1].bytes().next().unwrap() - b'a') as usize),
                "dec" => Self::Dec((components[1].bytes().next().unwrap() - b'a') as usize),
                "jnz" => { 
                    let input = components[1].parse::<usize>();
                    let target = components[2].parse::<isize>();
                    match (input, target) {
                        (Ok(0), _) => Self::Jmp(1),
                        (Ok(_), Ok(offset)) => Self::Jmp(offset),
                        (Ok(i), Err(_)) => Self::JmpReg((components[2].bytes().next().unwrap() - b'a') as usize, i as isize),
                        (Err(_), Ok(offset)) => Self::Jnz((components[1].bytes().next().unwrap() - b'a') as usize, offset),
                        (Err(_), Err(_)) => Self::Jnz((components[1].bytes().next().unwrap() - b'a') as usize, components[2].parse().unwrap()), 
                    }
                },
                "tgl" => Self::Tgl((components[1].bytes().next().unwrap() - b'a') as usize),
                "out" => Self::Out((components[1].bytes().next().unwrap() - b'a') as usize),
                _ => panic!("Unknown Instruction: {line}"),
            }
        }
    }

    pub struct Cpu {
        registers: [isize; 4],
        programm: Vec<Instruction>,
        instr_ptr: usize,
    }

    impl Cpu {
        pub fn new(assembly: &str) -> Self {
            Self {
                registers: [0; 4],
                instr_ptr: 0,
                programm: assembly.lines().map(Instruction::parse).collect(),
            }
        }

        pub fn reset(&mut self) {
            self.registers = [0; 4];
            self.instr_ptr = 0;
        }

        pub fn set(&mut self, register: usize, value: isize) {
            self.registers[register] = value;
        }

        pub fn get(&mut self, register: usize) -> isize {
            self.registers[register]
        }

        pub fn clone_volatile_state(&self) -> ([isize; 4], usize) {
            (self.registers, self.instr_ptr)
        }

        pub fn run(&mut self) -> Option<isize> {
            let mut state = State::Running;
            while state == State::Running {
                state = self.step();
            }
            if let State::Output(res) = state {
                Some(res)
            } else {
                None
            }
        }

        fn step(&mut self) -> State {
            let instruction = &self.programm[self.instr_ptr];
            self.instr_ptr += 1;
            match instruction {
                Instruction::CpyInt(i, to) => self.registers[*to] = *i,
                Instruction::CpyReg(from, to) => self.registers[*to] = self.registers[*from],
                Instruction::Inc(reg) => self.registers[*reg] += 1,
                Instruction::Dec(reg) => self.registers[*reg] -= 1,
                Instruction::Jnz(reg, offset) => {
                    // Special Case to speed up Addition
                    if *offset == -2 && self.instr_ptr>2 {
                        match (&self.programm[self.instr_ptr-3], &self.programm[self.instr_ptr-2]) {
                            (Instruction::Inc(other), Instruction::Dec(reg)) | (Instruction::Dec(reg), Instruction::Inc(other)) => {
                                self.registers[*other] += self.registers[*reg];
                                self.registers[*reg] = 0;
                            },
                            _ => if self.registers[*reg] != 0 { self.instr_ptr = (self.instr_ptr as isize + offset - 1) as usize; }
                        }
                    }
                    // Special Case to speed up Multiplication
                    if *offset == -5 && self.instr_ptr>5 {
                        match (&self.programm[self.instr_ptr-6], &self.programm[self.instr_ptr-5], &self.programm[self.instr_ptr-4], &self.programm[self.instr_ptr-3], &self.programm[self.instr_ptr-2]) {
                            (Instruction::CpyReg(b, c1), Instruction::Inc(a), Instruction::Dec(c2), Instruction::Jnz(c3, -2), Instruction::Dec(reg2)) |
                            (Instruction::CpyReg(b, c1), Instruction::Dec(c2), Instruction::Inc(a), Instruction::Jnz(c3, -2), Instruction::Dec(reg2)) if c1 == c2 && c1 == c3 && reg == reg2 && a != b=> {
                                self.registers[*a] += self.registers[*b] * self.registers[*reg];
                                self.registers[*reg] = 0;
                            },
                            (Instruction::CpyInt(i, c1), Instruction::Inc(a), Instruction::Dec(c2), Instruction::Jnz(c3, -2), Instruction::Dec(reg2)) |
                            (Instruction::CpyInt(i, c1), Instruction::Dec(c2), Instruction::Inc(a), Instruction::Jnz(c3, -2), Instruction::Dec(reg2)) if c1 == c2 && c1 == c3 && reg == reg2 => {
                                self.registers[*a] += i * self.registers[*reg];
                                self.registers[*reg] = 0;
                            },
                            _ => if self.registers[*reg] != 0 { self.instr_ptr = (self.instr_ptr as isize + offset - 1) as usize; },
                        }
                    }
                    if self.registers[*reg] != 0 { 
                    self.instr_ptr = (self.instr_ptr as isize + offset - 1) as usize; 
                    }
                }, 
                Instruction::JnzReg(reg, to) => if self.registers[*reg] != 0 {
                    self.instr_ptr = (self.instr_ptr as isize + self.registers[*to] - 1) as usize;
                },
                Instruction::Jmp(offset) => self.instr_ptr = (self.instr_ptr as isize + offset - 1) as usize,
                Instruction::JmpReg(reg, _) => self.instr_ptr = (self.instr_ptr as isize + self.registers[*reg] -1) as usize,
                Instruction::Tgl(reg_offset) => {
                    let target = self.registers[*reg_offset] + self.instr_ptr as isize - 1;
                    if (0..self.programm.len() as isize).contains(&target) {
                        let target = target as usize;
                        let old = &self.programm[target];
                        self.programm[target] = match old {
                            Instruction::Inc(reg) => Instruction::Dec(*reg),
                            Instruction::Dec(reg) => Instruction::Inc(*reg),
                            Instruction::Jmp(_) => Instruction::Jmp(1),
                            Instruction::Tgl(reg) => Instruction::Inc(*reg),
                            Instruction::Out(reg) => Instruction::Inc(*reg),
                            Instruction::JnzReg(reg, offset) => Instruction::CpyReg(*reg, *offset),
                            Instruction::CpyReg(from, to) => Instruction::JnzReg(*from, *to),
                            Instruction::CpyInt(i, to) => if *i==0 {
                                Instruction::Jmp(1)
                            } else {
                                Instruction::JmpReg(*to, *i)
                            },
                            Instruction::Jnz(_, _) => Instruction::Jmp(1),
                            Instruction::JmpReg(reg, i) => Instruction::CpyInt(*i, *reg)
                        }
                    }
                },
                Instruction::Out(reg) => return State::Output(self.registers[*reg]),
            }
            if self.instr_ptr >= self.programm.len() {
                State::Halt
            } else {
                State::Running
            }
        }
    }
}
