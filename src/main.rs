use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
}

struct Module {
    mass : i32,
}

fn read_module(file_name : &'static str) -> Vec<Module> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut modules = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mass: i32 = line.trim().parse().unwrap();
        let module = Module { mass };
        modules.push(module);
    }
    modules
}

fn fuel_1a(module : &Module) -> i32 {
    let fuel = module.mass / 3 - 2;
    fuel 
}

#[test]
fn day1a() {
    let modules = read_module("day1.txt");
    let total = modules.iter().fold(0, |acc, x| acc + fuel_1a(x));
    println!("Total: {}", total);
}

fn fuel_1b(module: &Module) -> i32 {
    fn fuel(mass: i32) -> i32 {
        let f = mass / 3 - 2;
        if f <= 0 {
            0
        } else {
            f + fuel(f)
        }
    }

    fuel(module.mass)
}   

#[test]
fn day1b() {
    let modules = read_module("day1.txt");
    let total = modules.iter().fold(0, |acc, x| acc + fuel_1b(x));
    println!("Total: {}", total);
}

fn read_program(file_name : &'static str) -> Vec<i32> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let line = reader.lines().nth(0).unwrap().unwrap();
    let ks = line.split(",");
    let r:Vec<i32> = ks.map(|x| { x.parse::<i32>().unwrap() }).collect();
    r
}

fn execute_program(program : &mut Vec<i32>) {
    let mut pc = 0;
    while program[pc] != 99 {
        let i0 = program[pc + 1] as usize;
        let i1 = program[pc + 2] as usize;
        let d = program[pc + 3] as usize;
        match program[pc] {
            1 => {
                program[d] = program[i0] + program[i1];
                pc = pc + 4;
            },
            2 => {
                program[d] = program[i0] * program[i1];
                pc = pc + 4;
            },
            _ => {
                println!("Illegal Instruction");
                break;
            }
        }
    }
}

fn run_test_program(program : &mut Vec<i32>, expected : &Vec<i32>) {
    execute_program(program);
    assert_eq!(expected, program);
}

#[test]
fn day2a_test_programs() {
    {
        let mut p : Vec<i32> = vec![1,0,0,0,99];
        let p_prime : Vec<i32> = vec![2,0,0,0,99];
        run_test_program(&mut p, &p_prime);
    }
    {
        let mut p : Vec<i32> = vec![2,3,0,3,99];
        let p_prime : Vec<i32> = vec![2,3,0,6,99];
        run_test_program(&mut p, &p_prime);
    }
    {
        let mut p : Vec<i32> = vec![2,4,4,5,99,0];
        let p_prime : Vec<i32> = vec![2,4,4,5,99,9801];
        run_test_program(&mut p, &p_prime);
    }
    {
        let mut p : Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        let p_prime : Vec<i32> = vec![30,1,1,4,2,5,6,0,99];
        run_test_program(&mut p, &p_prime);
    }
    {
        let mut p : Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let p_prime : Vec<i32> = vec![3500i32, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        run_test_program(&mut p, &p_prime);
    }
}

#[test]
fn day2a() {
    let mut program = read_program("day2.txt");
    program[1] = 12;
    program[2] = 2;
    execute_program(&mut program);
    println!("Program[0]: {}", program[0]);
}

#[test]
fn day2b() {
    let original_program = read_program("day2.txt");

    for len in 0.. {
        for noun in 0..(len + 1) {
            let verb = len - noun;

            if noun < original_program.len() && verb < original_program.len() {
                let mut program = original_program.clone();
                program[1] = noun as i32;
                program[2] = verb as i32;
                execute_program(&mut program);

                if program[0] == 19690720 {
                    assert!(false, "noun: {}, verb: {}", noun, verb);
                    return;
                }
            }
        }
    }
}