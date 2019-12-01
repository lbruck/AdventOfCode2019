use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
}

struct Module {
    mass : i32,
}

fn read_input(file_name : String) -> Vec<Module> {
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
    let modules = read_input("input.txt".to_string());
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
    let modules = read_input("input.txt".to_string());
    let total = modules.iter().fold(0, |acc, x| acc + fuel_1b(x));
    println!("Total: {}", total);
}