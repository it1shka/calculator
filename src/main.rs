use std::iter::Peekable;
use std::str::Chars;
use std::io::stdin;
use std::fmt;

fn int(string: &String) -> i32{
    string.parse().unwrap()
}

fn float(string: &String) -> f64{
    string.parse().unwrap()
}

#[derive(Debug)]
enum Token { 
    Add, 
    Sub, 
    Mul, 
    Div, 
    Pow, 
    Int(i32), 
    Float(f64), 
    LeftParen, 
    RightParen
}

impl Token {
    fn prec(&self) -> Option<u8> {
        match *self {
            Token::Add | Token::Sub => Some(1),
            Token::Mul | Token::Div => Some(2),
            Token::Pow              => Some(3),
            _                       => None
        }
    }

    fn is_op(&self) -> bool {
        match *self {
            Token::Add 
            | Token::Sub
            | Token::Mul
            | Token::Div
            | Token::Pow
                => true,
            _ => false
        }
    }

    fn is_num(&self) -> bool {
        match *self {
            Token::Int(_) | Token::Float(_) => true,
            _ => false
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::Add => write!(f, "Add (+)"),
            Token::Sub => write!(f, "Sub (-)"),
            Token::Mul => write!(f, "Mul (*)"),
            Token::Div => write!(f, "Div (/)"),
            Token::Pow => write!(f, "Pow (^)"),
            Token::Int(x) => write!(f, "Int ({})", x),
            Token::Float(x) => write!(f, "Float ({})", x),
            Token::LeftParen => write!(f, "Left Paren (\"(\")"),
            Token::RightParen => write!(f, "Right Paren (\")\")")
        }
    }
}

struct TokenStream<'a> {
    stream: Peekable<Chars<'a>>,
}

impl <'a> TokenStream<'a> {

    fn new(input: &String) -> TokenStream {
        TokenStream {
            stream: input.chars().peekable()
        }
    }

    fn read(&mut self) -> Option<Token> {
        self.read_while(|x| x == ' ');
        let current = self.stream.peek();
        match current {
            None => None,
            Some(c) => match c {
                '+' => { self.stream.next(); Some(Token::Add) },
                '-' => { self.stream.next(); Some(Token::Sub) },
                '*' => { self.stream.next(); Some(Token::Mul) },
                '/' => { self.stream.next(); Some(Token::Div) },
                '^' => { self.stream.next(); Some(Token::Pow) },
                '(' => { self.stream.next(); Some(Token::LeftParen) },
                ')' => { self.stream.next(); Some(Token::RightParen) },
                _ => {
                    if c.is_digit(10){
                        Some(self.read_number())
                    }
                    else {
                        panic!("Unexpected input: {}", c);
                    }
                }
            }
        }
    }

    fn get_digits(&mut self) -> String {
        self.read_while(|x| x.is_digit(10))
    }

    fn read_number(&mut self) -> Token{
        let integer_part = self.get_digits();
        let current = self.stream.peek();
        match current {
            None => Token::Int(int(&integer_part)),
            Some(x) => match x{
                '.' => {
                    let mut number = String::new();
                    number.push_str(&integer_part[..]);
                    number.push(self.stream.next().unwrap());
                    number.push_str(&self.get_digits()[..]);
                    Token::Float(float(&number))
                },
                _ => Token::Int(int(&integer_part))
            }
        }
    }

    fn read_while(&mut self, condition: impl Fn(char) -> bool ) -> String{
        let mut value = String::new();
        while let Some(current) = self.stream.peek() {
            if condition(*current) {
                value.push(*current);
                self.stream.next();
            }
            else {
                break;
            }
        }
        value
    }

}

impl Iterator for TokenStream<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}

struct ShuntingYard<'a> {
    stream: TokenStream<'a>,
}

impl<'a> ShuntingYard<'a> {
    fn new(stream: TokenStream) -> ShuntingYard {
        ShuntingYard { stream }
    }

    fn get_stack(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut op_stack: Vec<Token> = Vec::new();

        for token in &mut self.stream {
            if token.is_num(){
                tokens.push(token);
            }
            else if token.is_op(){
                while let Some(operator) = op_stack.last() {
                    if operator.is_op() && 
                        operator.prec().unwrap() >= token.prec().unwrap(){
                        tokens.push(op_stack.pop().unwrap());
                    }
                    else {
                        break;
                    }
                }
                op_stack.push(token);
            }
            else if let Token::LeftParen = token {
                op_stack.push(token);
            }
            else if let Token::RightParen = token {
                let mut escaped = false;
                while let Some(operator) = op_stack.pop() {
                    if let Token::LeftParen = operator {
                        escaped = true;
                        break;
                    }
                    else {
                        tokens.push(operator);
                    }
                }
                if !escaped {
                    panic!("Expected {}", Token::LeftParen);
                }
            }
            else {
                panic!("Unexpected token: {}", token);
            }
        }

        while let Some(operator) = op_stack.pop() {
            if let Token::LeftParen = operator {
                panic!("Unexpected {}", Token::LeftParen);
            }
            tokens.push(operator);
        }

        tokens
    }

}

fn pow(a: f64, b: i32) -> f64 {
    if b <= 0 { 1_f64 }
    else { a * pow(a, b - 1) }
}

fn operation_result(operator: Token, a: Token, b: Token) -> Token{
    let operation: fn(f64, f64) -> f64;
    match operator {
        Token::Add => operation = |a, b| a + b,
        Token::Sub => operation = |a, b| a - b,
        Token::Mul => operation = |a, b| a * b,
        Token::Div => operation = |a, b| a / b,
        Token::Pow => operation = |a, b| pow(a, b as i32),
        _ => operation = |a, b| 0_f64
    }

    match a{
        Token::Int(x) => {
            match b {
                Token::Int(y) => Token::Int(
                    operation(x as f64, y as f64) as i32),
                Token::Float(y) => Token::Float(
                    operation(x as f64, y)),
                _ => Token::Int(0)
            }
        },
        Token::Float(x) => {
            match b {
                Token::Int(y) => 
                    Token::Float(operation(x, y as f64)),
                Token::Float(y) => 
                    Token::Float(operation(x, y)),
                _ => Token::Int(0)
            }
        },
        _ => Token::Int(0)
    }

}

fn run_stack(stack: Vec<Token>){
    let mut mem: Vec<Token> = Vec::new();
    for token in stack {
        if token.is_num(){
            mem.push(token);
        }
        else if token.is_op(){
            let b = mem.pop().expect("Operator must have two args");
            let a = mem.pop().expect("Operator must have two args");
            mem.push(operation_result(token, a, b));
        }
    }

    let len = mem.len();
    if len == 0 {
        println!("0 (empty)");
    }
    else if len == 1 {
        match mem.pop().unwrap(){
            Token::Int(x) => println!("{}", x),
            Token::Float(x) => println!("{}", x),
            _ => {}
        }
    }
    else {
        panic!("Incorrect input!");
    }
}

fn main(){
    let mut user_input = String::new();
    loop {
        stdin().read_line(&mut user_input).unwrap();
        user_input = String::from(user_input.trim_end());

        if user_input == "exit" {
            return;
        }

        let stream = TokenStream::new(&user_input);
        let mut shunting_yard = ShuntingYard::new(stream);
        let stack = shunting_yard.get_stack();
        run_stack(stack);

        user_input.clear();
    }
}