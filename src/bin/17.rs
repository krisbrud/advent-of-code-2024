use itertools::Itertools;

advent_of_code::solution!(17);

struct Instruction {
    opcode: i64,
    operand: i64,
}

#[derive(Clone, Copy)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
}

impl Computer {
    fn combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved! Should not appear in valid programs"),
            _ => panic!("unexpected combo operand"),
        }
    }
}

struct StepOutput {
    next_state: Computer,
    output: Option<i64>,
    instruction_pointer: Option<usize>,
}

// The adv instruction (opcode 0) performs division.
// The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand.
// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
// The result of the division operation is truncated to an integer and then written to the A register.
fn adv(computer: &Computer, instruction: Instruction) -> StepOutput {
    let numerator = computer.a;
    let combo_operand = computer.combo_operand(instruction.operand);
    let denominator = 2i64.pow(
        combo_operand
            .try_into()
            .expect("Should convert combo operand"),
    );
    let result = numerator / denominator;
    let mut next_state = computer.clone();
    next_state.a = result;
    StepOutput {
        next_state,
        output: None,
        instruction_pointer: None,
    }
}

// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
// then stores the result in register B.
fn bxl(computer: &Computer, instruction: Instruction) -> StepOutput {
    let result = computer.b ^ instruction.operand;
    let mut next_state = computer.clone();
    next_state.b = result;
    StepOutput {
        next_state,
        output: None,
        instruction_pointer: None,
    }
}

// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
fn bst(computer: &Computer, instruction: Instruction) -> StepOutput {
    let combo_operand = computer.combo_operand(instruction.operand);
    let result = combo_operand % 8;
    let mut next_state = computer.clone();
    next_state.b = result;
    StepOutput {
        next_state,
        output: None,
        instruction_pointer: None,
    }
}

// The jnz instruction (opcode 3) does nothing if the A register is 0.
// However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
// if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
fn jnz(computer: &Computer, instruction: Instruction) -> StepOutput {
    let next_state = computer.clone();
    if computer.a == 0 {
        StepOutput {
            next_state,
            output: None,
            instruction_pointer: None,
        }
    } else {
        StepOutput {
            next_state,
            output: None,
            instruction_pointer: Some(
                instruction
                    .operand
                    .try_into()
                    .expect("Should convert instruction pointer!"),
            ),
        }
    }
}

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
// then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc(computer: &Computer, instruction: Instruction) -> StepOutput {
    let result = computer.b ^ computer.c;
    let mut next_state = computer.clone();
    next_state.b = result;
    StepOutput {
        next_state,
        output: None,
        instruction_pointer: None,
    }
}

// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
// (If a program outputs multiple values, they are separated by commas.)
fn out(computer: &Computer, instruction: Instruction) -> StepOutput {
    let combo_operand = computer.combo_operand(instruction.operand);
    let result = combo_operand % 8;
    let mut next_state = computer.clone();
    StepOutput {
        next_state,
        output: Some(result),
        instruction_pointer: None,
    }
}

// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is
// stored in the B register. (The numerator is still read from the A register.)
fn bdv(computer: &Computer, instruction: Instruction) -> StepOutput {
    let numerator = computer.a;
    let combo_operand = computer.combo_operand(instruction.operand);
    let denominator = 2i64.pow(
        combo_operand
            .try_into()
            .expect("Should convert combo operand"),
    );
    let result = numerator / denominator;
    let mut next_state = computer.clone();
    next_state.b = result;
    StepOutput {
        next_state,
        output: None,
        instruction_pointer: None,
    }
}

// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the
// C register. (The numerator is still read from the A register.)
fn cdv(computer: &Computer, instruction: Instruction) -> StepOutput {
    let numerator = computer.a;
    let combo_operand = computer.combo_operand(instruction.operand);
    let denominator = 2i64.pow(
        combo_operand
            .try_into()
            .expect("Should convert combo operand"),
    );
    let result = numerator / denominator;
    let mut next_state = computer.clone();
    next_state.c = result;
    StepOutput {
        next_state,
        output: None,
        instruction_pointer: None,
    }
}

fn execute_instruction(computer: &Computer, instruction: Instruction) -> StepOutput {
    match instruction.opcode {
        0 => adv(computer, instruction),
        1 => bxl(computer, instruction),
        2 => bst(computer, instruction),
        3 => jnz(computer, instruction),
        4 => bxc(computer, instruction),
        5 => out(computer, instruction),
        6 => bdv(computer, instruction),
        7 => cdv(computer, instruction),
        _ => panic!("Invalid opcode"),
    }
}

fn parse(input: &str) -> Option<(Computer, Vec<i64>)> {
    let a: i64 = input.lines().nth(0)?.split(" ").last()?.parse().ok()?;
    let b: i64 = input.lines().nth(1)?.split(" ").last()?.parse().ok()?;
    let c: i64 = input.lines().nth(2)?.split(" ").last()?.parse().ok()?;

    let computer = Computer { a, b, c };

    let instructions: Vec<i64> = input
        .lines()
        .nth(4)?
        .split(" ")
        .last()?
        .split(",")
        .map(|x| x.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    Some((computer, instructions))
}

pub fn part_one(input: &str) -> Option<String> {
    let (initial_computer, instructions) = parse(input).expect("Should parse input!");

    let (final_computer, outputs) = simulate(initial_computer, instructions);

    let out_string: String = outputs
        .iter()
        .map(|output| output.to_string())
        .join(",")
        .to_string();

    Some(out_string)
}

fn simulate(initial_computer: Computer, instructions: Vec<i64>) -> (Computer, Vec<i64>) {
    let mut computer = initial_computer.clone();
    let mut instruction_pointer: usize = 0;
    let mut outputs: Vec<i64> = vec![];

    while instruction_pointer < instructions.len() {
        let (opcode, operand) = instructions
            .clone()
            .into_iter()
            .tuple_windows::<(_, _)>()
            .nth(instruction_pointer)
            .expect("Should find instruction");
        let instruction = Instruction { opcode, operand };

        let step_output = execute_instruction(&computer, instruction);

        if let Some(output) = step_output.output {
            outputs.push(output);
        }
        instruction_pointer = step_output
            .instruction_pointer
            .unwrap_or(instruction_pointer + 2);
        computer = step_output.next_state;
    }
    (computer, outputs)
}

// 0 : adv - write to a
// 1 : bxl - write to b
// 2 : bst - write to b
// 3 : jnz - jump
// 4 : bxc - write to b
// 5 : out - output
// 6 : bdv - read a, write to b
// 7 : cdv - read a, write to c

// 0 is only that writes to a
// 7 is only that writes to c


pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn test_first_example() {
        let (computer, outputs) = simulate(Computer { a: 0, b: 0, c: 9 }, vec![2, 6]);
        assert_eq!(1, computer.b);
    }

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn test_second_example() {
        let (computer, outputs) = simulate(Computer { a: 10, b: 0, c: 0 }, vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(vec![0, 1, 2], outputs);
    }

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn test_third_example() {
        let (computer, outputs) = simulate(
            Computer {
                a: 2024,
                b: 0,
                c: 0,
            },
            vec![0, 1, 5, 4, 3, 0],
        );
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], outputs);
        assert_eq!(0, computer.a);
    }

    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn test_fourth_example() {
        let (computer, outputs) = simulate(Computer { a: 0, b: 29, c: 0 }, vec![1, 7]);
        assert_eq!(26, computer.b);
    }

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn test_fifth_example() {
        let (computer, outputs) = simulate(Computer { a: 0, b: 2024, c: 43690 }, vec![4, 0]);
        assert_eq!(44354, computer.b);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
