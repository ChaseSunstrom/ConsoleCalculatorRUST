use std::*;
use evalexpr::*;

fn main() {

    println!("Welcome to the calculator! Make sure to enter a valid mathematical operation. And add a space after each character/number.");

    loop{
        println!("Enter a mathematical operation:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let result = eval(&input).unwrap();

        let extracted_result = match result {
            Value::Int(x) => x,
            Value::Float(x) => x.round() as i64,
            _ => panic!("Error parsing value.")
        };

        println!("{:?}" , extracted_result);

        println!("Break? (y/n)");
        let mut question = String::new();
        io::stdin().read_line(&mut question).expect("Failed to read line");
        if question.contains('y')  {
            break;
        } else {
            continue;
        }
    }
}
