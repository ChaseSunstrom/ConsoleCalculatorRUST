use std::*;
use evalexpr::*;

fn main() {

    println!("Welcome to the calculator! Make sure to enter a valid mathematical operation. And add a space after each character/number. You can only enter in 2 numbers at a time.");

    loop{
        println!("Enter a mathematical operation.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut result = eval(&input);
        println!("{:?}" , result);

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
