
use std::io::{stdin,stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
     .expect("failed to flush");
    stdin().read_line(input)
     .expect("failed to read");
}

fn main()
{
println!("calculate");
println!("--------");
loop{
let mut n1 = String::new();
let mut n2 = String::new();
let mut op = String::new();

println!("enter first number ?:",);
read(&mut n1);

println!("enter second number ?:",);
read(&mut n2);

println!("enter operation you want to perform ? [*+/-]:");
read(&mut op);

let n1: f32 = n1.trim().parse().unwrap();
let n2: f32 = n2.trim().parse().unwrap();
let op: char = op.trim().chars().next().unwrap();

let ops = String::from("*+/-");

if !ops.contains(op){
    println!("unknown operator");
    continue;

}
let result = match op {
    '*' => n1 * n2,
    '+' => n1 + n2,
    '/' => n1 / n2,
    '-' => n1 - n2,
    _ => panic!("error in operator")
};
println!("{} {} {} ={}",n1, op,n2,result);
}
}
