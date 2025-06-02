use std::{error::Error, io::Write};

use simple_calculator::Calculator;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let mut calculator = Calculator::new(input);
        println!("{}", calculator.calculate().unwrap());
    }
}
