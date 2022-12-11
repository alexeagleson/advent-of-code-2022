use std::collections::VecDeque;

pub mod input;

const DEBUG_MODE: bool = false;
const PRINT_SIGNAL_STRENGTH: bool = false;

struct CRT {
    pixels: Vec<Vec<char>>,
}

impl CRT {
    pub fn draw(&mut self, x: usize, y: usize) {
        if x > 39 {
            panic!("Draw outside X bounds {}", x);
        } else if y > 5 {
            panic!("Draw outside Y bounds {}", y);
        }

        // These X/Y values are swapped intentionally because of how the vectors are setup
        self.pixels[y][x] = '#';
    }

    pub fn new() -> Self {
        let line = vec!['.'; 40];

        Self {
            pixels: vec![line.clone(); 6],
        }
    }

    pub fn print_it(&self) {
        for row in &self.pixels {
            for pixel in row {
                print!("{}", pixel);
            }
            print!("\n");
        }
    }
}

#[derive(Debug)]
struct CPU {
    register: i32,
    active_operation: Option<ActiveOperation>,
    cycle_count: i32,
    signal_strength_history: Vec<i32>,
}

#[derive(Debug, PartialEq)]

enum Operation {
    Addx(i32),
    Noop,
}

#[derive(Debug, PartialEq)]
struct ActiveOperation {
    operation: Operation,
    current_cycle: i32,
}

impl ActiveOperation {
    pub fn new(operation: Operation) -> Self {
        Self {
            operation,
            current_cycle: 1,
        }
    }
}

impl Operation {
    pub fn cycles_requires(&self) -> i32 {
        match self {
            Operation::Addx(_) => 2,
            Operation::Noop => 1,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register: 1,
            active_operation: None,
            cycle_count: 1,
            signal_strength_history: Vec::new(),
        }
    }

    pub fn current_screen_row(&self) -> usize {
        ((self.cycle_count - 1) / 40) as usize
    }

    pub fn current_screen_column(&self) -> usize {
        ((self.cycle_count - 1) % 40) as usize
    }

    pub fn signal_strength(&self) -> i32 {
        self.register * self.cycle_count
    }

    pub fn signal_strength_at_cycle(&self, cycle: usize) -> i32 {
        let strength = self.signal_strength_history.get(cycle - 2);
        match strength {
            Some(strength) => *strength,
            None => {
                panic!("Requested signal strength at invalid cycle {}", cycle);
            }
        }
    }

    pub fn set_active_operation(&mut self, operation: Operation) {
        if DEBUG_MODE {
            println!("Setting new operation {:?}", operation);
        }
        if self.active_operation.is_some() {
            panic!(
                "Attempted to set a new operation while another is currently active {:?}",
                self.active_operation
            );
        }
        self.active_operation = Some(ActiveOperation::new(operation));
    }

    pub fn process_operation(&mut self) {
        self.cycle_count += 1;

        match self.active_operation {
            Some(ref mut active_operation) => {
                if DEBUG_MODE {
                    println!("Processing operation {:?}", active_operation);
                }

                if active_operation.current_cycle < active_operation.operation.cycles_requires() {
                    if DEBUG_MODE {
                        println!("Incrementing current operation cycle");
                    }
                    active_operation.current_cycle += 1;
                    return;
                } else if active_operation.current_cycle
                    == active_operation.operation.cycles_requires()
                {
                    match active_operation.operation {
                        Operation::Addx(add_value) => {
                            self.register += add_value;
                        }
                        Operation::Noop => {
                            // Do nothing
                        }
                    }

                    if DEBUG_MODE {
                        println!("Current operation complete");
                    }
                    self.active_operation = None;
                } else {
                    panic!(
                        "Current operation cycle is higher than required cycles {:?}",
                        active_operation
                    );
                }
            }
            None => panic!("Requested to process work with no active operations"),
        }
    }
}

fn parse_input(input: &str) -> VecDeque<Operation> {
    let mut operations = VecDeque::new();
    for line in input.lines() {
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            continue;
        }

        let split = trimmed_line.split_once(" ");

        let operation = match split {
            Some(split) => match split {
                ("addx", val) => {
                    Operation::Addx(i32::from_str_radix(val, 10).expect("Invalid addx value"))
                }
                _ => panic!(
                    "Encountered a multi word operation text that does not match addx {}",
                    trimmed_line
                ),
            },
            None => match trimmed_line {
                "noop" => Operation::Noop,
                _ => panic!(
                    "Encountered a single operation text that does not match noop {}",
                    trimmed_line
                ),
            },
        };

        operations.push_back(operation);
    }
    operations
}

fn process_operations(
    cpu: &mut CPU,
    mut operation_queue: VecDeque<Operation>,
    mut crt: Option<&mut CRT>,
) {
    if DEBUG_MODE {
        println!("Starting processing operations");
    }

    while let Some(next_operation) = operation_queue.pop_front() {
        cpu.set_active_operation(next_operation);

        while cpu.active_operation.is_some() {
            if let Some(crt) = crt.as_deref_mut() {
                let x = cpu.current_screen_column();
                let y = cpu.current_screen_row();

                if x >= (cpu.register - 1) as usize && x <= (cpu.register + 1) as usize {
                    crt.draw(x, y);
                }
            }

            cpu.process_operation();

            cpu.signal_strength_history.push(cpu.signal_strength());

            if PRINT_SIGNAL_STRENGTH {
                if cpu.cycle_count % 20 == 0 {
                    println!(
                        "Cycle count is {} and register value is {} and signal strength is {}",
                        cpu.cycle_count,
                        cpu.register,
                        cpu.signal_strength()
                    );
                }
            }
        }
    }

    if DEBUG_MODE {
        println!("Finished processing operations");
    }
}

pub fn day_10_part_1(input: &'static str) -> String {
    let operations = parse_input(input);

    let mut cpu = CPU::new();

    process_operations(&mut cpu, operations, None);

    let relevant_strengths = vec![
        cpu.signal_strength_at_cycle(20),
        cpu.signal_strength_at_cycle(60),
        cpu.signal_strength_at_cycle(100),
        cpu.signal_strength_at_cycle(140),
        cpu.signal_strength_at_cycle(180),
        cpu.signal_strength_at_cycle(220),
    ];

    let strength_sum: i32 = relevant_strengths.iter().sum();

    strength_sum.to_string()
}

pub fn day_10_part_2(input: &'static str) -> String {
    let operations = parse_input(input);

    let mut cpu = CPU::new();
    let mut crt = CRT::new();

    process_operations(&mut cpu, operations, Some(&mut crt));

    crt.print_it();

    "TBA".to_string()
}

#[cfg(test)]
mod test {

    use crate::day_10::input::LONG_TEST_INPUT;

    use super::*;

    // Register should be -1 after these operations
    static TEST_INPUT: &str = r#"
        noop
        addx 3
        addx -5
    "#;

    #[test]
    fn test_parse_input() {
        let operations = parse_input(TEST_INPUT);

        let op1 = Operation::Noop;
        let op2 = Operation::Addx(3);
        let op3 = Operation::Addx(-5);

        assert_eq!(operations, vec![op1, op2, op3]);
    }

    #[test]
    fn test_cpu_register() {
        let operations = parse_input(TEST_INPUT);

        let mut cpu = CPU::new();

        process_operations(&mut cpu, operations, None);

        assert_eq!(cpu.register, -1);
    }

    #[test]
    fn test_signal_strength() {
        let operations = parse_input(LONG_TEST_INPUT);

        let mut cpu = CPU::new();

        process_operations(&mut cpu, operations, None);

        assert_eq!(cpu.signal_strength_at_cycle(20), 420);

        let relevant_strengths = vec![
            cpu.signal_strength_at_cycle(20),
            cpu.signal_strength_at_cycle(60),
            cpu.signal_strength_at_cycle(100),
            cpu.signal_strength_at_cycle(140),
            cpu.signal_strength_at_cycle(180),
            cpu.signal_strength_at_cycle(220),
        ];

        dbg!("{:?}", &relevant_strengths);

        let strength_sum: i32 = relevant_strengths.iter().sum();

        assert_eq!(strength_sum, 13140);
    }

    #[test]
    fn test_print_crt() {
        let mut crt = CRT::new();

        crt.draw(30, 2);

        crt.print_it();

        assert_eq!(crt.pixels[2][30], '#');
    }
}
