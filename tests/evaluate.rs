use simple_calculator::*;

#[test]
fn test_basic_addition() {
    let mut calc = Calculator::new("1 + 2".to_string());
    assert_eq!(calc.calculate(), 3.0);
}

#[test]
fn test_basic_subtraction() {
    let mut calc = Calculator::new("5 - 3".to_string());
    assert_eq!(calc.calculate(), 2.0);
}

#[test]
fn test_basic_multiplication() {
    let mut calc = Calculator::new("2 * 3".to_string());
    assert_eq!(calc.calculate(), 6.0);
}

#[test]
fn test_basic_division() {
    let mut calc = Calculator::new("6 / 2".to_string());
    assert_eq!(calc.calculate(), 3.0);
}

#[test]
fn test_basic_modulo() {
    let mut calc = Calculator::new("7 % 3".to_string());
    assert_eq!(calc.calculate(), 1.0);
}

#[test]
fn test_operator_precedence() {
    let mut calc = Calculator::new("2 + 3 * 4".to_string());
    assert_eq!(calc.calculate(), 14.0);
}

#[test]
fn test_parentheses() {
    let mut calc = Calculator::new("(2 + 3) * 4".to_string());
    assert_eq!(calc.calculate(), 20.0);
}

#[test]
fn test_nested_parentheses() {
    let mut calc = Calculator::new("(2 + (3 * 2)) * 4".to_string());
    assert_eq!(calc.calculate(), 32.0);
}

#[test]
fn test_negative_numbers() {
    let mut calc = Calculator::new("-3 + 5".to_string());
    assert_eq!(calc.calculate(), 2.0);
}

#[test]
fn test_decimal_numbers() {
    let mut calc = Calculator::new("1.5 + 2.5".to_string());
    assert_eq!(calc.calculate(), 4.0);
}

#[test]
fn test_complex_expression() {
    let mut calc = Calculator::new("(3 + 5) * 2 - 8 / 4 + 10 % 3".to_string());
    assert_eq!(calc.calculate(), 15.0);
}

#[test]
#[should_panic(expected = "Unmatched parenthesis")]
fn test_unmatched_parenthesis() {
    let mut calc = Calculator::new("(2 + 3".to_string());
    calc.calculate();
}

#[test]
#[should_panic(expected = "Unexpected end of input")]
fn test_empty_expression() {
    let mut calc = Calculator::new("".to_string());
    calc.calculate();
}

#[test]
#[should_panic(expected = "Unexpected token: &")]
fn test_invalid_token() {
    let mut calc = Calculator::new("2 & 3".to_string());
    println!("{}", calc.calculate());
}
