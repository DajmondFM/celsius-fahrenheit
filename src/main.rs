use std::io;
fn c_to_f() {
    let c = input();
    let mut f:f32= 32.0;
    f = f+c*1.8;
    println!("{}", f.round());
}
// 1c == 1.8f

fn f_to_c() {
    let f = input();
    let c:f32= (f-32.0)/1.8;
    println!("{}", c.round());
}

fn input() -> f32 {
    let mut number = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");
    let x: f32 = number.trim().parse().expect("Invalid number");
    return x;
}


use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("No argument given. Try cf to convert celsius to fahrenheit and fc to convert fahrenheit to celsius."),
        2 =>    match args[1].as_str() {
                "cf" => c_to_f(),
                "fc" => f_to_c(),
                _ => println!("Type cf to convert celsius to fahrenheit and fc to convert fahrenheit to celsius."),
                }
        _ => println!("Too many arguments."),
        
    }
}