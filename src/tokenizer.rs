#[derive(Debug)]
pub enum Token {
    Paren(Paren),
    Name(Name),
    Number(Number),
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            Token::Paren(v) => v.value.clone(),
            Token::Name(v) => v.value.clone(),
            Token::Number(v) => v.value.clone(),
        }
    }
}

// impl TryInto<Paren> for Token {
//     type Error = String;

//     fn try_into(self) -> Result<Paren, Self::Error> {
//         match self {
//             Token::Paren(token) => Ok(token),
//             _ => Err(String::from("Can't convert {:?token} into Paren")),
//         }
//     }
// }

#[derive(Debug)]
pub struct Paren {
    pub value: String,
}

impl Paren {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct Name {
    pub value: String,
}

impl Name {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct Number {
    pub value: String,
}

impl Number {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = vec![];
    let mut current: usize = 0;
    while current < chars.len() {
        let mut ch = chars[current];
        if ch == '(' || ch == ')' {
            let value = String::from(ch);
            tokens.push(Token::Paren(Paren::new(value)));
            current += 1;
            continue;
        }
        if ch.is_ascii_alphabetic() {
            let mut value = String::new();
            while ch.is_ascii_alphabetic() {
                value.push(ch);
                current += 1;
                ch = chars[current];
            }
            tokens.push(Token::Name(Name::new(value)));
            continue;
        }
        if ch.is_ascii_digit() {
            let mut value = String::new();
            while ch.is_ascii_digit() {
                value.push(ch);
                current += 1;
                ch = chars[current];
            }
            tokens.push(Token::Number(Number::new(value)));
            continue;
        }
        if ch.is_ascii_whitespace() {
            current += 1;
            continue;
        }

        return Err(format!("Invalid char: {}", ch));
    }
    Ok(tokens)
}
