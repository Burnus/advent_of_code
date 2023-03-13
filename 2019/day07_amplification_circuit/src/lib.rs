use std::{sync::{mpsc, Arc}, thread, collections::VecDeque, num::ParseIntError};

use intcode_processor::intcode_processor::{Cpu, OutputState};

pub fn run(input: &str) -> Result<(isize, isize), ParseIntError> {
    let template = Cpu::try_with_memory_from_str(input)?;
    let mut first = 0;
    for perm in get_permutations(&(0..5).collect()) {
        let mut output = 0;
        for input in perm.iter() {
            let mut amp = template.clone();
            amp.set_input(*input);
            amp.set_input(output);
            loop {
                match amp.run() {
                    OutputState::DiagnosticCode(out) => {
                            output = out;
                            break;
                        },
                    OutputState::Output(e) => amp.set_input(e),
                    OutputState::Halt => break,
                }
            }
        }
        first = first.max(output);
    }
    let mut second = 0;
    for perm in get_permutations(&(5..=9).collect()) {
        let output = Arc::new(std::sync::Mutex::new(0));
        let mut receivers = VecDeque::new();
        let mut transmitters = VecDeque::new();
        let last = perm.len()-1;
        for _ in 0..=last {
            let (t, r) = mpsc::channel::<isize>();
            receivers.push_back(r);
            transmitters.push_back(t);
        }
        transmitters[0].send(0).unwrap();
        transmitters.rotate_left(1);

        let mut handles = Vec::new();
        for (amp_idx, phase) in perm.into_iter().enumerate() {
            let tx = transmitters.pop_front().unwrap();
            let rx = receivers.pop_front().unwrap();
            let output = Arc::clone(&output);
            let mut local_out = 0;
            let mut amp = template.clone();
            let handle = thread::spawn(move || {
                amp.set_input(phase);
                loop {
                    amp.set_input(rx.recv().unwrap());
                    match amp.run() {
                        OutputState::Output(out) => {
                            local_out = out;
                            tx.send(out).unwrap(); 
                        },
                        OutputState::Halt => {
                            // Swallow errors, because at this point we are halting anyway, so we
                            // don't care about whether or not the next amp has already
                            // deconstructed anymore.
                            let _ = tx.send(0);
                            break;
                        },
                        OutputState::DiagnosticCode(d) => { 
                            local_out = d;
                            // Swallow errors, because at this point we are halting anyway, so we
                            // don't care about whether or not the next amp has already
                            // deconstructed anymore.
                            let _ = tx.send(d);
                            break;
                        },
                    }
                }
                if amp_idx == last {
                    *output.lock().unwrap() = local_out;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()
        }
        second = second.max(*output.lock().unwrap());
    }
    Ok((first, second))
}

fn get_permutations(numbers: &Vec<isize>) -> Vec<Vec<isize>> {
    if numbers.len() == 1 {
        vec![numbers.to_vec()]
    } else {
        let mut res = Vec::new();
        for (idx, n) in numbers.iter().enumerate() {
            let mut rest = numbers.to_vec();
            rest.remove(idx);
            let rest_perms = get_permutations(&rest);
            for mut p in rest_perms {
                p.push(*n);
                res.push(p);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        // The second part of the output is not verified, but that's what my solution produces.
        assert_eq!(run(&sample_input), Ok((65210, 76543)));
    }

    #[test]
    fn test_sample_2() {
        let sample_input = read_file("tests/sample_input_2");
        // The first part of the output is not verified, but that's what my solution produces.
        assert_eq!(run(&sample_input), Ok((0, 18216)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input), Ok((46248, 54163586)));
    }
}
