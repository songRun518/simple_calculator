use crate::Token;

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

        let chars = src.chars().collect::<Vec<_>>();
        let mut num_start: isize = -1;
        let mut num_end: isize = -1;
        for (idx, chr) in chars.iter().enumerate() {
            if chr.is_whitespace() {
                continue;
            }

            if chr.is_ascii_digit() || chr == &'.' {
                if num_start == num_end {
                    num_start = idx as isize;
                }
                continue;
            }

            let token = match chr {
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Div,
                '%' => Token::Mod,
                '(' => Token::LParen,
                ')' => Token::RParen,
                other => Token::Invalid(*other),
            };

            if num_start == num_end {
                num_start = idx as isize;
                num_end = idx as isize;
            } else {
                num_end = idx as isize;
                let number = src[num_start as usize..num_end as usize]
                    .trim()
                    .parse::<f64>()
                    .unwrap();
                tokens.push(Token::Number(number));
                num_start = num_end;
            }

            tokens.push(token);
        }
        if num_start != num_end {
            num_end = src.len() as isize;
            let number = src[num_start as usize..num_end as usize]
                .trim()
                .parse::<f64>()
                .unwrap();
            tokens.push(Token::Number(number));
        }

        tokens
    }
}

impl Calculator {
    pub fn calculate(&mut self) -> f64 {
        let mut result = self.parse_term();

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                Token::Add => {
                    self.pos += 1;
                    result += self.parse_term();
                }
                Token::Sub => {
                    self.pos += 1;
                    result -= self.parse_term();
                }
                Token::Invalid(c) => panic!("Unexpected token: {}", c), // 新增对Invalid的处理
                _ => break,
            }
        }

        result
    }

    fn parse_term(&mut self) -> f64 {
        let mut result = self.parse_factor();

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                Token::Mul => {
                    self.pos += 1;
                    result *= self.parse_factor();
                }
                Token::Div => {
                    self.pos += 1;
                    result /= self.parse_factor();
                }
                Token::Mod => {
                    self.pos += 1;
                    result %= self.parse_factor();
                }
                Token::Invalid(c) => panic!("Unexpected token: {}", c), // 新增对Invalid的处理
                _ => break,
            }
        }

        result
    }

    fn parse_factor(&mut self) -> f64 {
        if self.pos >= self.tokens.len() {
            panic!("Unexpected end of input");
        }

        match self.tokens[self.pos] {
            Token::Number(n) => {
                self.pos += 1;
                n
            }
            Token::LParen => {
                self.pos += 1;
                let result = self.calculate();
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RParen {
                    panic!("Unmatched parenthesis");
                }
                self.pos += 1;
                result
            }
            Token::Sub => {
                self.pos += 1;
                -self.parse_factor()
            }
            Token::Invalid(c) => panic!("Unexpected token: {}", c), // 专门处理无效字符
            _ => panic!("Unexpected token: {:?}", self.tokens[self.pos]),
        }
    }
}
