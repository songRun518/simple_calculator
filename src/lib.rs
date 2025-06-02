#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LParen,
    RParen,
    Invalid(char),
}

pub struct Calculator {
    tokens: Vec<Token>,
    pos: usize,
}

impl Calculator {
    pub fn new(src: String) -> Self {
        let tokens = Self::parse_tokens(src);
        Self { tokens, pos: 0 }
    }

    pub fn parse_tokens(src: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = src.chars().peekable();
        let mut buffer = String::new();

        while let Some(chr) = chars.next() {
            if chr.is_whitespace() {
                continue;
            }

            if chr.is_ascii_digit() || chr == '.' {
                buffer.push(chr);
                while let Some(&next_chr) = chars.peek() {
                    if next_chr.is_ascii_digit() || next_chr == '.' {
                        buffer.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if let Ok(number) = buffer.parse::<f64>() {
                    tokens.push(Token::Number(number));
                    buffer.clear();
                } else {
                    tokens.push(Token::Invalid(chr));
                }
                continue;
            }

            if !buffer.is_empty() {
                if let Ok(number) = buffer.parse::<f64>() {
                    tokens.push(Token::Number(number));
                    buffer.clear();
                } else {
                    tokens.push(Token::Invalid(chr));
                }
            }

            let token = match chr {
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Div,
                '%' => Token::Mod,
                '(' => Token::LParen,
                ')' => Token::RParen,
                other => Token::Invalid(other),
            };
            tokens.push(token);
        }

        if !buffer.is_empty() {
            if let Ok(number) = buffer.parse::<f64>() {
                tokens.push(Token::Number(number));
            } else {
                tokens.push(Token::Invalid(buffer.chars().next().unwrap()));
            }
        }

        tokens
    }

    pub fn calculate(&mut self) -> Result<f64, String> {
        let result = self.parse_expression()?;

        if self.pos < self.tokens.len() {
            return Err(format!("Unexpected token at position {}", self.pos));
        }

        Ok(result)
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut result = self.parse_term()?;

        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                Some(Token::Add) => {
                    self.pos += 1;
                    result += self.parse_term()?;
                }
                Some(Token::Sub) => {
                    self.pos += 1;
                    result -= self.parse_term()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut result = self.parse_factor()?;

        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                Some(Token::Mul) => {
                    self.pos += 1;
                    result *= self.parse_factor()?;
                }
                Some(Token::Div) => {
                    self.pos += 1;
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= divisor;
                }
                Some(Token::Mod) => {
                    self.pos += 1;
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err("Modulo by zero".to_string());
                    }
                    result %= divisor;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        if self.pos >= self.tokens.len() {
            return Err("Unexpected end of input".to_string());
        }

        match self.tokens[self.pos] {
            Token::Number(n) => {
                self.pos += 1;
                Ok(n)
            }
            Token::LParen => {
                self.pos += 1;
                let result = self.parse_expression()?;
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RParen {
                    return Err("Unmatched parenthesis".to_string());
                }
                self.pos += 1;
                Ok(result)
            }
            Token::Sub => {
                self.pos += 1;
                Ok(-self.parse_factor()?)
            }
            Token::Invalid(c) => Err(format!("Unexpected token: {}", c)),
            _ => Err(format!("Unexpected token: {:?}", self.tokens[self.pos])),
        }
    }
}
