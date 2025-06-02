use simple_calculator::*;

#[test]
fn test_parse_tokens() {
    let tokens = Calculator::parse_tokens("1 + 2.5 * (3 - 4)".to_string());
    assert_eq!(
        tokens,
        vec![
            Token::Number(1.0),
            Token::Add,
            Token::Number(2.5),
            Token::Mul,
            Token::LParen,
            Token::Number(3.0),
            Token::Sub,
            Token::Number(4.0),
            Token::RParen,
        ]
    );

    let tokens = Calculator::parse_tokens("1.2.3".to_string());
    assert!(matches!(tokens[0], Token::Invalid(_))); // 无效数字格式
}

#[test]
fn test_simple_addition() {
    let mut calc = Calculator::new("1 + 2".to_string());
    assert_eq!(calc.calculate(), Ok(3.0));

    let mut calc = Calculator::new("1 + 2 + 3 + 4".to_string());
    assert_eq!(calc.calculate(), Ok(10.0));
}

#[test]
fn test_subtraction() {
    let mut calc = Calculator::new("5 - 3".to_string());
    assert_eq!(calc.calculate(), Ok(2.0));

    let mut calc = Calculator::new("10 - 2 - 3".to_string());
    assert_eq!(calc.calculate(), Ok(5.0));
}

#[test]
fn test_multiplication() {
    let mut calc = Calculator::new("2 * 3".to_string());
    assert_eq!(calc.calculate(), Ok(6.0));

    let mut calc = Calculator::new("2 * 3 * 4".to_string());
    assert_eq!(calc.calculate(), Ok(24.0));
}

#[test]
fn test_division() {
    let mut calc = Calculator::new("6 / 2".to_string());
    assert_eq!(calc.calculate(), Ok(3.0));

    let mut calc = Calculator::new("10 / 2 / 5".to_string());
    assert_eq!(calc.calculate(), Ok(1.0));
}

#[test]
fn test_division_by_zero() {
    let mut calc = Calculator::new("5 / 0".to_string());
    assert_eq!(calc.calculate(), Err("Division by zero".to_string()));
}

#[test]
fn test_modulo() {
    let mut calc = Calculator::new("5 % 2".to_string());
    assert_eq!(calc.calculate(), Ok(1.0));

    let mut calc = Calculator::new("10 % 3 % 2".to_string());
    assert_eq!(calc.calculate(), Ok(1.0));
}

#[test]
fn test_modulo_by_zero() {
    let mut calc = Calculator::new("5 % 0".to_string());
    assert_eq!(calc.calculate(), Err("Modulo by zero".to_string()));
}

#[test]
fn test_operator_precedence() {
    let mut calc = Calculator::new("2 + 3 * 4".to_string());
    assert_eq!(calc.calculate(), Ok(14.0));

    let mut calc = Calculator::new("(2 + 3) * 4".to_string());
    assert_eq!(calc.calculate(), Ok(20.0));
}

#[test]
fn test_parentheses() {
    let mut calc = Calculator::new("(1 + 2) * 3".to_string());
    assert_eq!(calc.calculate(), Ok(9.0));

    let mut calc = Calculator::new("1 + (2 * 3) + 4".to_string());
    assert_eq!(calc.calculate(), Ok(11.0));

    let mut calc = Calculator::new("((1 + 2) * (3 + 4))".to_string());
    assert_eq!(calc.calculate(), Ok(21.0));
}

#[test]
fn test_unmatched_parentheses() {
    let mut calc = Calculator::new("(1 + 2".to_string());
    assert_eq!(calc.calculate(), Err("Unmatched parenthesis".to_string()));

    let mut calc = Calculator::new("1 + 2)".to_string());
    assert!(calc.calculate().is_err());
}

#[test]
fn test_negative_numbers() {
    let mut calc = Calculator::new("-5 + 3".to_string());
    assert_eq!(calc.calculate(), Ok(-2.0));

    let mut calc = Calculator::new("5 * -3".to_string());
    assert_eq!(calc.calculate(), Ok(-15.0));

    let mut calc = Calculator::new("-(-3)".to_string());
    assert_eq!(calc.calculate(), Ok(3.0));
}

#[test]
fn test_decimal_numbers() {
    let mut calc = Calculator::new("1.5 + 2.5".to_string());
    assert_eq!(calc.calculate(), Ok(4.0));

    let mut calc = Calculator::new("0.1 + 0.2".to_string());
    assert!((calc.calculate().unwrap() - 0.3).abs() < f64::EPSILON);
}

#[test]
fn test_invalid_tokens() {
    let mut calc = Calculator::new("1 @ 2".to_string());
    assert!(calc.calculate().is_err());

    let mut calc = Calculator::new("abc".to_string());
    assert!(calc.calculate().is_err());
}

#[test]
fn test_whitespace_handling() {
    let mut calc = Calculator::new("  1   +   2   ".to_string());
    assert_eq!(calc.calculate(), Ok(3.0));

    let mut calc = Calculator::new("\t1\n+\r2".to_string());
    assert_eq!(calc.calculate(), Ok(3.0));
}

#[test]
fn test_complex_expressions() {
    let mut calc = Calculator::new("(3 + 5) * (7 - 2) / 4 % 3".to_string());
    assert_eq!(calc.calculate(), Ok(1.0));

    let mut calc = Calculator::new("-2 * (3 + 4) - 5 / (1 + 1)".to_string());
    assert_eq!(calc.calculate(), Ok(-16.5));
}
