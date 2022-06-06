use std::io;

fn read(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Failed to read from stdin");
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Enter the first digit:");
    read(&mut num1);
    let num1:f32 = num1.trim().parse().expect("Not a valid number");

    println!("Enter the operator:");
    read(&mut operator);
    let operator:char = operator.trim().chars().next().unwrap();

    println!("Enter the second digit:");
    read(&mut num2);
    let num2:f32 = num2.trim().parse().expect("Not a valid number");
    
    let result:f32 = match operator  {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => if num2 == 0.0 {
                    println!("Cannot divide by 0");
                    return;
                } else {num1 / num2}
        _ => panic!("Not a valid operator")
    };

    print!("\n{}{}{}={}", num1, operator, num2, result);
}