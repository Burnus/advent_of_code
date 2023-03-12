pub mod intcode_processor {
    use std::{collections::VecDeque, num::ParseIntError};

    type RegVal = isize;

    /// The return conditions of the Cpu:
    /// - Output(RegVal) if some return instruction ocurred mid-program (without a Halt immediately
    /// succeeding it),
    /// - DiagnosticCode(RegVal) if a return instruction with a Halt immediately after it was
    /// encountered, or
    /// - Halt if a Halt instruction was triggered that did not immediately follow an output
    /// instruction
    #[derive(PartialEq, Eq, Debug)]
    pub enum OutputState { Output(RegVal), DiagnosticCode(RegVal), Halt }

    /// The Cpu struct holds the state of our processor. It consists of `memory`, which serves
    /// instructions as well as data, and `instr_ptr`, a pointer to the next instruction. It starts at 0.
    /// Furthermore, there is an `input` register that is read by some instructions.
    /// Element 0 is assumed to contain a valid opcode. The subsequent elements will be treated 
    /// as its operands. After executing an instruction, instr_ptr will be incremented by the number of
    /// expected operands + 1 (for the instruction). Since instructions have different lengths, and
    /// there is nothing preventing an instruction from overwriting a memory address that will be
    /// considered an instruction later on, no guarantees can be given about the eventual memory
    /// layout without examining the entire program.
    ///
    /// `Cpu`s should be instantiated by `with_memory()` and their program executed by `run()`.
    #[derive(Default, Clone)]
    pub struct Cpu {
        memory: Vec<RegVal>,
        instr_ptr: usize,
        input: VecDeque<RegVal>,
        rel_base: RegVal,
    }

    impl Cpu {
        /// Initialize a new Processor using the given input as the memory. The Instruction Pointer
        /// starts at 0.
        ///
        /// ## Example:
        /// ````
        /// use intcode_processor::intcode_processor::Cpu;
        ///
        /// let mut cpu = Cpu::with_memory(vec![1, 2, 3, 4, 99]);
        /// assert_eq!(cpu.get(0), 1);
        /// assert_eq!(cpu.get(1), 2);
        /// assert_eq!(cpu.get(2), 3);
        /// assert_eq!(cpu.get(3), 4);
        /// assert_eq!(cpu.get(4), 99);
        /// ````
        pub fn with_memory(memory: Vec<RegVal>) -> Self {
            Self {
                memory,
                ..Default::default()
            }
        }

        pub fn try_with_memory_from_str(input: &str) -> Result<Self, ParseIntError> {
            let mem: Result<Vec<_>, _> = input.split(',').map(|s| s.parse::<RegVal>()).collect();
            match mem {
                Ok(memory) => Ok(Self {
                                    memory,
                                    ..Default::default()
                                }),
                Err(e) => Err(e),
            }
        }

        /// Set the value in memory address `address` to `value`.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::Cpu;
        ///
        /// let mut cpu = Cpu::with_memory(vec![1, 0, 0, 0, 99]);
        /// cpu.set(0, 2);
        /// assert_eq!(cpu.get(0), 2);
        /// ````
        pub fn set(&mut self, address: usize, value: RegVal) {
            let len = self.memory.len();
            if address >= len {
                self.memory.append(&mut vec![0; address+1-len]);
            }
            self.memory[address] = value;
        }

        /// Sets the input to `input`. This will be read by certain commands, like opcode 3
        /// (set_to_input).
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// // Store input (opcode 3) register 4. Then Output (opcode 104) register 3 and Halt (opcode 99).
        /// let mut cpu = Cpu::with_memory(vec![3, 3, 104, 0, 99]);
        /// cpu.set_input(42);
        /// assert_eq!(cpu.run(), OutputState::DiagnosticCode(42));
        /// ````
        pub fn set_input(&mut self, input: RegVal) {
            self.input.push_back(input);
        }

        /// Get the value from memory address `address`.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::Cpu;
        ///
        /// let mut cpu = Cpu::with_memory(vec![1, 0, 0, 0, 99]);
        /// assert_eq!(cpu.get(0), 1);
        /// ````
        pub fn get(&self, address: usize) -> RegVal {
            if address < self.memory.len() {
                self.memory[address]
            } else {
                0
            }
        }


        /// Add val_1 and val_2 and store the result in address dest.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 1, 20, 21, 17,          // Add values from addresses 20 and 21 (23 + 42) and write to 17.
        /// 101, 23, 20, 18,        // Add 23 and value from 20 (23 + 23) and write to 18.
        /// 1001, 21, 42, 19,       // Add value from 21 and literal 42 (42 + 42) and write to 19.
        /// 1101, 100, -1, 16,      // Add literals 100 and -1 and write to 16.
        /// 0, 0, 0, 0, 23, 42]);   // 16-21: Output destinations and inputs.
        /// assert_eq!(cpu.run(), OutputState::Halt);
        /// assert_eq!(cpu.get(17), 65);
        /// assert_eq!(cpu.get(18), 46);
        /// assert_eq!(cpu.get(19), 84);
        /// ````
        fn add(&mut self, reg_1: usize, reg_2: usize, dest: usize) {
            // eprintln!("{val_1}+{val_2}={}", val_1+val_2);
            self.set(dest, self.get(reg_1) + self.get(reg_2));
            self.instr_ptr += 4;
        }

        /// Multiply val_1 and val_2 and store the result in dest.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 2, 20, 21, 17,          // Multiply values from addresses 20 and 21 (23 * 42) and write to 17.
        /// 102, 23, 20, 18,        // Multiply 23 and value from 20 (23 * 23) and write to 18.
        /// 1002, 21, 42, 19,       // Multiply value from 21 and literal 42 (42 * 42) and write to 19.
        /// 1102, -33, -3, 16,      // Multiply literals -33 and -3 and write to 16.
        /// 0, 0, 0, 0, 23, 42]);   // 16-21: Output destinations and inputs.
        /// assert_eq!(cpu.run(), OutputState::Halt);
        /// assert_eq!(cpu.get(17), 966);
        /// assert_eq!(cpu.get(18), 529);
        /// assert_eq!(cpu.get(19), 1764);
        /// ````
        fn mul(&mut self, reg_1: usize, reg_2: usize, dest: usize) {
            self.set(dest, self.get(reg_1) * self.get(reg_2));
            self.instr_ptr += 4;
        }

        /// Store the first element of `input` at address `dest`.
        ///
        /// ## Panics
        /// Panics if dest points outside the memory or input is empty.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::Cpu;
        ///
        /// // Store input (opcode 3) in register 4. Then Halt (opcode 99).
        /// let mut cpu = Cpu::with_memory(vec![3, 4, 99, 0, 0]);
        /// cpu.set_input(42);
        /// cpu.run();
        /// assert_eq!(cpu.get(4), 42);
        /// ````
        fn set_to_input(&mut self, dest: usize) {
            let input = self.input.pop_front().expect("Input is empty");
            self.set(dest, input);
            self.instr_ptr += 2;
        }

        /// Return DiagnosticCode(val) if the next instruction is Halt (opcode 99) and Output(val) otherwise.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// // Return (opcode 4) the value from memory address 3. Then Halt (opcode 99).
        /// let mut cpu = Cpu::with_memory(vec![104, 42, 99]);
        /// assert_eq!(cpu.run(), OutputState::DiagnosticCode(42));
        /// ````
        fn ret(&mut self, reg: usize) -> OutputState {
            self.instr_ptr += 2;
            if self.get(self.instr_ptr) == 99 {
                OutputState::DiagnosticCode(self.get(reg))
            } else {
                OutputState::Output(self.get(reg))
            }
        }

        /// Jump to address `dest_val`, if `val` is non-zero.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 1005, 0, 4, // jump to 4 if address 0 holds a non-zero value (it does, neamely 1005).
        /// 99,         // instruction 3: Skipped due to first jump
        /// 1105, 0, 10,// jump to 10 if 0 is non-zero (it isn't, so we continue)
        /// 104, 1,     // return literal 1
        /// 99,         // halt (this is where we actually halt)
        /// 104, 2,     // instruction 10: This would be our jump target if we didn't jump from 4; return literal 2.
        /// 99,         // halt
        /// ]);
        /// assert_eq!(cpu.run(), OutputState::DiagnosticCode(1));
        /// ````
        fn jnz(&mut self, reg: usize, dest_reg: usize) {
            if self.get(reg) != 0 {
                self.instr_ptr = self.get(dest_reg) as usize;
            } else {
                self.instr_ptr += 3;
            }
        }

        /// Jump to address `dest_val`, if `val` is zero.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 1106, 0, 4, // jump to 4 if 0 is zero (it is).
        /// 99,         // instruction 3: Skipped due to first jump
        /// 1006, 0, 10,// jump to 10 if address 0 holds a zero (it doesn't, so we continue)
        /// 104, 1,     // return literal 1
        /// 99,         // halt (this is where we actually halt)
        /// 104, 2,     // instruction 10: This would be our jump target if we didn't jump from 4; return literal 2.
        /// 99,         // halt
        /// ]);
        /// assert_eq!(cpu.run(), OutputState::DiagnosticCode(1));
        /// ````
        fn jiz(&mut self, reg: usize, dest_reg: usize) {
            if self.get(reg) == 0 {
                self.instr_ptr = self.get(dest_reg) as usize;
            } else {
                self.instr_ptr += 3;
            }
        }
 
        /// Set address `dest` to `1` if `val_1` is lower value than `val_2` and
        /// to `0` otherwise.
        ///
        /// ## Panics
        /// Panics if any of the addresses point outside the memory.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::Cpu;
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 1107, 23, 42, 3, // Set address 3 to 1, since 23 < 42
        /// 1107, 42, 23, 7, // Set address 7 to 0, since !(42<23)
        /// 7, 1, 1, 11,     // Set address 11 to 0, since address 1 doesn't hold a lower value than itself !(23<23)
        /// 99,              // Halt
        /// ]);
        ///
        /// cpu.run();
        /// assert_eq!(cpu.get(3), 1);
        /// assert_eq!(cpu.get(7), 0);
        /// assert_eq!(cpu.get(11), 0);
        /// ````
        fn lt(&mut self, reg_1: usize, reg_2: usize, dest: usize) {
            self.set(dest, 
            if self.get(reg_1) < self.get(reg_2) {
                1
            } else {
                0
            });
            self.instr_ptr += 4;
        }

        /// Set address `dest` to `1` if values `val_1` and `val_2` are equal
        /// does and to `0` otherwise.
        ///
        /// ## Panics
        /// Panics if any of the addresses point outside the memory.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::Cpu;
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 1108, 23, 23, 3,// Set address 3 to 1, since 23 == 23
        /// 1108, 23, 42, 7,// Set address 7 to 0, since (23 != 42)
        /// 8, 3, 7, 11,    // Set address 11 to 0, since 3 and 7 hold different values (1 != 0)
        /// 99,             // Halt
        /// 23, 42]);       // data storage (addresses 9 and 10)
        ///
        /// cpu.run();
        /// assert_eq!(cpu.get(3), 1);
        /// assert_eq!(cpu.get(7), 0);
        /// assert_eq!(cpu.get(11), 0);
        /// ````
        fn eq(&mut self, reg_1: usize, reg_2: usize, dest: usize) {
            self.set(dest, 
                     if self.get(reg_1) == self.get(reg_2) {
                         1
                     } else {
                         0
            });
            self.instr_ptr += 4;
        }

        /// Adjust the relative base pointer by the given amount.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// let mut cpu = Cpu::with_memory(vec![
        /// 109, -2,    // Decrease relative base by 1 (to -2),
        /// 204, 4,     // Output the contents of address (relative base + 4 == 2)
        /// 99          // Halt
        /// ]);
        ///
        /// assert_eq!(cpu.run(), OutputState::DiagnosticCode(204));
        /// ````
        fn adj_rel_base(&mut self, reg: usize) {
            self.rel_base += self.get(reg);
            self.instr_ptr += 2;
        }

        fn pos(&self, offset: usize) -> usize { 
            self.get(self.instr_ptr + offset) as usize 
        }
        fn imm(&self, offset: usize) -> usize { 
            self.instr_ptr + offset 
        }
        fn rel(&self, offset: usize) -> usize { 
            (self.rel_base + self.get(self.instr_ptr + offset)) as usize 
        } 

        /// Run the program from current memory, starting at `instr_ptr` and running until opcode
        /// 99 (Halt) is encountered.
        ///
        /// Returns DiagnosticCode(val) if the program encountered some output instruction immediately followed
        /// by a Halt (opcode 99), Halt if it ended on Halt without encountering an output
        /// instruction, or Output(val) on an output instruction that is not followed by a Halt.
        ///
        /// ## Panics
        /// Throws a `panic` whenever an undefined opcode is encountered at `instr_ptr`, or if the
        /// program is trying to access an address outside the memory.
        ///
        /// ## Example
        /// ````
        /// use intcode_processor::intcode_processor::{Cpu, OutputState};
        ///
        /// // Add (opcode 1) registers 5 (23) and 6 (42) and store into 0. Then Halt (opcode 99).
        /// let mut cpu = Cpu::with_memory(vec![1, 5, 6, 0, 99, 23, 42]);
        /// let result = cpu.run();
        /// assert_eq!(result, OutputState::Halt);
        /// assert_eq!(cpu.get(0), 65);
        /// ````
        pub fn run(&mut self) -> OutputState {
            loop {
                let instruction = self.memory[self.instr_ptr];
                // eprintln!("{}: {}", self.instr_ptr, instruction);
                let params: Vec<usize> = (1..=3).map(|i| match (instruction/(10_isize.pow(i+1)))%10 {
                    0 => self.pos(i as usize),
                    1 => self.imm(i as usize),
                    2 => self.rel(i as usize),
                    e => panic!("Unexpected mode: {e}"),
                }).collect();

                match instruction % 100 {
                    1 => self.add(params[0], params[1], params[2]),
                    2 => self.mul(params[0], params[1], params[2]),
                    3 => self.set_to_input(params[0]),
                    4 => return self.ret(params[0]),
                    5 => self.jnz(params[0], params[1]),
                    6 => self.jiz(params[0], params[1]),
                    7 => self.lt(params[0], params[1], params[2]),
                    8 => self.eq(params[0], params[1], params[2]),
                    9 => self.adj_rel_base(params[0]),
                    99 => return OutputState::Halt,
                    _ => panic!("Unexpected instruction: {}", instruction),
                }
            }
        }
        
    }
}


#[cfg(test)]
mod tests {
    use super::intcode_processor::*;

    #[test]
    fn day02_1() {
        let mem = vec![1,12,2,3,2,3,11,0,99,30,40,20,173,984523];
        let mut cpu = Cpu::with_memory(mem);
        assert_eq!(cpu.run(), OutputState::Halt);
        assert_eq!(cpu.get(0), 3500);
    }

    #[test]
    fn day02_2() {
        let mem = vec![1,2,13,3,2,3,11,0,99,30,40,20,173,984523];
        let mut cpu = Cpu::with_memory(mem);
        assert_eq!(cpu.run(), OutputState::Halt);
        assert_eq!(cpu.get(0), 19690720);
    }

    #[test]
    fn day05() {
        let mem = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        for input in 0..=10 {
            let mut cpu = Cpu::with_memory(mem.to_vec());
            cpu.set_input(input);
            let expected = match input {
                l if l < 8 => 999,
                8          => 1000,
                g if g > 8 => 1001,
                _ => unreachable!(),
            };

            assert_eq!(cpu.run(), OutputState::Output(expected));
        }
    }
    
    #[test]
    fn day07() {
        let template = Cpu::try_with_memory_from_str("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0").unwrap();
        let perm = [0, 1, 2, 3, 4];
        let mut output = 0;
        for input in perm.iter() {
            let mut cpu = template.clone();
            cpu.set_input(*input);
            cpu.set_input(output);
            loop {
                match cpu.run() {
                    OutputState::DiagnosticCode(out) => {
                        output = out;
                        break;
                    },
                    OutputState::Output(e) => cpu.set_input(e),
                    OutputState::Halt => break,
                }
            }
        }
        assert_eq!(output, 56012);
    }

    #[test]
    fn rel_mode() {
        let mem = [109, 10, 204, -7, 99];
        let mut cpu = Cpu::with_memory(mem.to_vec());

        assert_eq!(cpu.run(), OutputState::DiagnosticCode(-7));
    }

    #[test]
    fn day09_sample() {
        let mem = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut cpu = Cpu::with_memory(mem.to_vec());

        for res in mem {
            assert_eq!(cpu.run(), OutputState::Output(res));
        }
    }

    #[test]
    fn day09() {
        let mut cpu = Cpu::with_memory(vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,37,1000,1101,856,0,1029,1101,286,0,1025,1101,39,0,1004,1101,861,0,1028,1101,845,0,1026,1102,28,1,1002,1102,1,0,1020,1101,0,892,1023,1101,0,291,1024,1101,35,0,1018,1101,0,27,1006,1102,1,26,1011,1101,33,0,1019,1102,31,1,1014,1102,1,36,1010,1102,23,1,1007,1101,0,32,1016,1101,29,0,1008,1101,20,0,1001,1102,1,25,1015,1101,38,0,1017,1101,0,24,1012,1102,1,22,1005,1101,1,0,1021,1101,0,21,1003,1102,1,838,1027,1102,1,30,1013,1101,895,0,1022,1101,0,34,1009,109,7,1208,0,22,63,1005,63,201,1001,64,1,64,1105,1,203,4,187,1002,64,2,64,109,-6,2102,1,5,63,1008,63,24,63,1005,63,223,1105,1,229,4,209,1001,64,1,64,1002,64,2,64,109,17,21102,40,1,-6,1008,1012,40,63,1005,63,255,4,235,1001,64,1,64,1106,0,255,1002,64,2,64,109,-15,21108,41,41,9,1005,1012,277,4,261,1001,64,1,64,1106,0,277,1002,64,2,64,109,11,2105,1,10,4,283,1105,1,295,1001,64,1,64,1002,64,2,64,109,-9,21101,42,0,8,1008,1013,44,63,1005,63,315,1105,1,321,4,301,1001,64,1,64,1002,64,2,64,109,13,1206,3,337,1001,64,1,64,1106,0,339,4,327,1002,64,2,64,109,-10,1208,0,29,63,1005,63,361,4,345,1001,64,1,64,1106,0,361,1002,64,2,64,109,2,2108,27,-4,63,1005,63,383,4,367,1001,64,1,64,1105,1,383,1002,64,2,64,109,-4,1207,2,30,63,1005,63,405,4,389,1001,64,1,64,1105,1,405,1002,64,2,64,109,22,1205,-8,417,1106,0,423,4,411,1001,64,1,64,1002,64,2,64,109,-27,2108,19,0,63,1005,63,443,1001,64,1,64,1106,0,445,4,429,1002,64,2,64,109,13,21108,43,45,-1,1005,1013,461,1106,0,467,4,451,1001,64,1,64,1002,64,2,64,109,1,21107,44,45,4,1005,1019,485,4,473,1105,1,489,1001,64,1,64,1002,64,2,64,109,-8,2102,1,-7,63,1008,63,37,63,1005,63,515,4,495,1001,64,1,64,1106,0,515,1002,64,2,64,109,1,2107,38,-4,63,1005,63,533,4,521,1105,1,537,1001,64,1,64,1002,64,2,64,109,4,21107,45,44,1,1005,1013,553,1106,0,559,4,543,1001,64,1,64,1002,64,2,64,109,-7,2107,21,-4,63,1005,63,575,1106,0,581,4,565,1001,64,1,64,1002,64,2,64,109,9,1205,7,599,4,587,1001,64,1,64,1105,1,599,1002,64,2,64,109,-11,2101,0,-3,63,1008,63,40,63,1005,63,619,1105,1,625,4,605,1001,64,1,64,1002,64,2,64,109,1,2101,0,-2,63,1008,63,28,63,1005,63,651,4,631,1001,64,1,64,1106,0,651,1002,64,2,64,109,1,21102,46,1,7,1008,1012,44,63,1005,63,671,1106,0,677,4,657,1001,64,1,64,1002,64,2,64,109,4,1201,-7,0,63,1008,63,28,63,1005,63,699,4,683,1105,1,703,1001,64,1,64,1002,64,2,64,109,-6,1207,-3,36,63,1005,63,719,1105,1,725,4,709,1001,64,1,64,1002,64,2,64,109,-4,1201,6,0,63,1008,63,23,63,1005,63,745,1106,0,751,4,731,1001,64,1,64,1002,64,2,64,109,8,1202,-6,1,63,1008,63,20,63,1005,63,777,4,757,1001,64,1,64,1105,1,777,1002,64,2,64,109,5,1202,-5,1,63,1008,63,25,63,1005,63,801,1001,64,1,64,1105,1,803,4,783,1002,64,2,64,109,8,21101,47,0,-6,1008,1014,47,63,1005,63,829,4,809,1001,64,1,64,1106,0,829,1002,64,2,64,109,1,2106,0,6,1001,64,1,64,1106,0,847,4,835,1002,64,2,64,109,11,2106,0,-4,4,853,1105,1,865,1001,64,1,64,1002,64,2,64,109,-15,1206,3,883,4,871,1001,64,1,64,1106,0,883,1002,64,2,64,109,14,2105,1,-8,1105,1,901,4,889,1001,64,1,64,4,64,99,21102,1,27,1,21102,1,915,0,1106,0,922,21201,1,57564,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1105,1,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1105,1,922,22201,1,-1,-2,1106,0,968,21202,-2,1,-2,109,-3,2106,0,0]);
        cpu.set_input(1);

        assert_eq!(cpu.run(), OutputState::DiagnosticCode(2316632620));
    }
}
