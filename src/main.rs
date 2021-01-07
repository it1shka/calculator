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
    fn precedence(&self) -> u8 {
        match *self {
            Token::Add | Token::Sub => 1,
            Token::Mul | Token::Div => 2,
            Token::Pow => 3,
            _ => 4
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Add => write!(f, "Add"),
            Token::Sub => write!(f, "Sub"),
            Token::Mul => write!(f, "Mul"),
            Token::Div => write!(f, "Div"),
            Token::Pow => write!(f, "Pow"),
            Token::Int(x) => write!(f, "Int {}", x),
            Token::Float(x) => write!(f, "Float {}", x),
            Token::LeftParen => write!(f, "Left Paren"),
            Token::RightParen => write!(f, "Right Paren")
        }
    }
}

struct TokenStream<'a> {
    stream: Peekable<Chars<'a>>
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

struct Engine<'b> {
    stream: TokenStream<'b>,
    num_stack: Vec<Token>,
    op_stack: Vec<Token>
}

impl<'b> Engine<'b> {

    fn new(stream: TokenStream) -> Engine{
        Engine{
            stream,
            num_stack: Vec::new(),
            op_stack: Vec::new()
        }
    }

    fn run(&mut self){
        while let Some(token) = self.stream.read() {
            match token {
                Token::Int(_) | Token::Float(_) => 
                    self.num_stack.push(token),
                Token::LeftParen => 
                    self.op_stack.push(token),
                Token::RightParen => 
                    self.on_right_paren(),
                Token::Add 
                | Token::Sub 
                | Token::Mul 
                | Token::Div 
                | Token::Pow => 
                    self.on_operator(token)
            }
        }
        self.clear_stack();
        if self.num_stack.len() > 1 {
            panic!("Incorrect input! Len(num_stack) > 1");
        }
        else if self.num_stack.len() == 0 {
            println!("Expression is empty");
        } 
        else {
            let last_token = self.num_stack.pop().unwrap();
            match last_token {
                Token::Float(x) => println!("{}", x),
                Token::Int(x) => println!("{}", x),
                _ => panic!("Illegal token on the top of the stack: {}", last_token)
            }
        }
    }

    fn  on_right_paren(&mut self){
        while let Some(token) = self.op_stack.pop(){
            match token {
                Token::LeftParen => return,
                
            }
        }
        panic!("Expected \"(\"");
    }

    fn on_operator(&mut self, operator: Token){

    }

    fn clear_stack(&mut self){

    }

    fn peform_operation(&mut self, operator: Token)

}

fn main(){
    let mut user_input = String::new();
    loop {
        stdin().read_line(&mut user_input).unwrap();
        user_input = String::from(user_input.trim_end());
        let mut stream = TokenStream::new(&user_input);
        let mut engine = Engine::new(stream);
        engine.run();
        user_input.clear();
    }
}