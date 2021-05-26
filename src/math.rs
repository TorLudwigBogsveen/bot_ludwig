/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

pub struct TokenTree {
    base: TokenNode
}

impl TokenTree {
    pub fn new(equation: &str) -> TokenTree {
        TokenTree {
            base: parse_part(&tokenize(equation)),
        }
    }

    pub fn sum(&self) -> f64 {
        self.base.sum()
    }
}

struct TokenNode {
    token: Token,
    lhs: Option<Box<TokenNode>>,
    rhs: Option<Box<TokenNode>>,
}

impl TokenNode {
    fn with_token(token: Token) -> TokenNode {
        TokenNode {
            token,
            lhs: None,
            rhs: None,
        }
    }

    fn sum(&self) -> f64 {
        let token = self.token;
        match token {
            Token::Plus     => return self.lhs.as_ref().unwrap().sum() + self.rhs.as_ref().unwrap().sum(),
            Token::Minus    => return self.lhs.as_ref().unwrap().sum() - self.rhs.as_ref().unwrap().sum(),
            Token::Multiply => return self.lhs.as_ref().unwrap().sum() * self.rhs.as_ref().unwrap().sum(),
            Token::Divide   => return self.lhs.as_ref().unwrap().sum() / self.rhs.as_ref().unwrap().sum(),
            Token::Power    => return self.lhs.as_ref().unwrap().sum().powf(self.rhs.as_ref().unwrap().sum()),
    
            Token::Num(num) => return num,
            _ => {
                println!("Error while sumating the equation!!!");
                return 1.0
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Token {
    None,

    Plus,
    Minus,
    Multiply,
    Divide,
    Power,

    LeftParens,
    RightParens,

    Equals,
    
    Num(f64)
}

impl Token {
    fn print(&self) {
        match self {
            Token::None => print!("None!"),
    
            Token::Plus     => print!("+"),
            Token::Minus    => print!("-"),
            Token::Multiply => print!("*"),
            Token::Divide   => print!("/"),
            Token::Power    => print!("^"),
    
            Token::LeftParens  => print!("("),
            Token::RightParens => print!(")"),
    
            Token::Equals => print!("="),
    
            Token::Num(num) => print!("{}", num),
        }
    }
}

#[derive(Copy, Clone)]
enum ParseIndex {
    Lhs,
    Rhs,
    Operator,
    Done,
}

fn parse_part(tokens: &[Token]) -> TokenNode {
    //print!("tokens: ");
    //print_tokens(tokens);

    let mut token_node = TokenNode {
        token: Token::None,
        lhs: None,
        rhs: None
    };

    let mut parse_index = ParseIndex::Lhs;
    let mut index = 0;
    while index < tokens.len() {
        let token = tokens[index];
        /*print!("PRINT SINGLE TOKEN: ");
        print_token(&token);
        println!();*/
        match (token, parse_index) {
            (Token::Num(_), ParseIndex::Lhs) => { token_node.lhs = Some(Box::new(TokenNode::with_token(token))); parse_index = ParseIndex::Operator },
            (Token::Num(_), ParseIndex::Rhs) => { token_node.rhs = Some(Box::new(TokenNode::with_token(token))); parse_index = ParseIndex::Done },

            (Token::Minus, ParseIndex::Lhs) => { 
                index += 1; 
                match tokens[index] {
                    Token::Num(num) => {
                        token_node.lhs = Some(Box::new(TokenNode::with_token(Token::Num(-num))));
                        parse_index = ParseIndex::Operator 
                    }
                    _ => assert!(false) 
                }
            },
            (Token::Minus, ParseIndex::Rhs) => { 
                index += 1; 
                match tokens[index] {
                    Token::Num(num) => {
                        token_node.rhs = Some(Box::new(TokenNode::with_token(Token::Num(-num))));
                        parse_index = ParseIndex::Operator 
                    }
                    _ => assert!(false) 
                }
            },

            (Token::Power, ParseIndex::Operator) |
            (Token::Plus, ParseIndex::Operator) |
            (Token::Minus, ParseIndex::Operator) |
            (Token::Multiply, ParseIndex::Operator) |
            (Token::Divide, ParseIndex::Operator) => { token_node.token = token; parse_index = ParseIndex::Rhs;
                /*print!("PRINT SINGLE TOKEN: ");
                print_token(&token);
                println!();*/
            },
            
            (Token::Power, ParseIndex::Done) |
            (Token::Plus, ParseIndex::Done) |
            (Token::Minus, ParseIndex::Done) |
            (Token::Multiply, ParseIndex::Done) |
            (Token::Divide, ParseIndex::Done) => {
                let mut new_token_node = TokenNode::with_token(token);
                new_token_node.lhs = Some(Box::new(token_node));
                token_node = new_token_node;
                parse_index = ParseIndex::Rhs;
            }

            (Token::LeftParens, ParseIndex::Lhs) => {
                let bracket_width = bracket_width(&tokens[index..]);
                //println!("LHS");
                //println!("bw: {}", bracket_width);
                //print_tokens(&tokens[(index+1)..(index+bracket_width-1)]);
                token_node.lhs = Some(Box::new(parse_part(&tokens[(index+1)..(index+bracket_width-1)]))); 
                parse_index = ParseIndex::Operator;
                index += bracket_width-1;
            }

            (Token::LeftParens, ParseIndex::Rhs) => {
                let bracket_width = bracket_width(&tokens[index..]);
                //println!("RHS");
                //println!("bw: {}", bracket_width);
                //print_tokens(&tokens[(index+1)..(index+bracket_width-1)]);
                token_node.rhs = Some(Box::new(parse_part(&tokens[(index+1)..(index+bracket_width-1)]))); 
                parse_index = ParseIndex::Done;
                index += bracket_width-1;
            }

            _ => {}
        }
        index += 1;
    }
    //println!("end of parse");
    token_node
}

fn tokenize(part: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = part.chars().collect(); 

    let mut index = 0;
    while index < part.len() {
        let c = chars[index];
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '^' => tokens.push(Token::Power),

            '(' => tokens.push(Token::LeftParens),
            ')' => tokens.push(Token::RightParens),

            '=' => tokens.push(Token::Equals),

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { 
                let (i, num) = parse_num(&chars[index..]);
                index += i-1;
                tokens.push(Token::Num(num));
            },
            _ => {}
        }
        index += 1;
    }

    tokens
}

fn bracket_width(tokens: &[Token]) -> usize {
    let mut n_brackets = 0;
    let mut width = 0;
    for token in tokens {
        width += 1;
        match token {
            Token::LeftParens => n_brackets += 1,
            Token::RightParens => {
                n_brackets -= 1;
                if n_brackets == 0 { return width } 
            }
            _ => {}
        }
        
    }
    
    println!("Error could'nt find matching right bracket!!!");
    0
}

fn parse_num(part: &[char]) -> (usize, f64) {
    let mut num = String::new();
    let mut index = 0;
    while index < part.len() {
        let c = part[index];
        index += 1;
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => { 
                num.push(c);
            },
            _ => {
                index = part.len();
            }
        }
    }

    (num.len(), num.parse().unwrap())
}