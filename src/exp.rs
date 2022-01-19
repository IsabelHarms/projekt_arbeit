
// AST
enum Exp {
    Int {
        val: i32,
    },
    Plus {
        left: Box<Exp>,  // Box = heap allocated necessary due to recursive definition
        right: Box<Exp>,
    },
    Mult {
        left: Box<Exp>,
        right: Box<Exp>,
    },
}

// Show for expressions.
fn show_exp(x : &Exp) -> String {
    match x {
        Exp::Int{val} => { return val.to_string(); }
        Exp::Plus{left, right} => { let s = "(".to_string() + &show_exp(&left)
                                             + &"+".to_string() + &show_exp(&right) + &")".to_string();
                                    return s; }
        Exp::Mult{left, right} => { let s = "(".to_string() + &show_exp(&left)
                                            + &"*".to_string() + &show_exp(&right) + &")".to_string();
                                    return s; }
    }
}

// Evaluation for expressions
fn eval_exp(x: &Exp) -> i32 {
  match x {
    Exp::Int{val} => *val,
    Exp::Plus{left, right} => eval_exp(&left)+eval_exp(&right),
    Exp::Mult{left, right} => eval_exp(&left)*eval_exp(&right),
  }
}


//Tokenizer
#[derive(PartialEq)]
enum Token {
    PLUS, MULT, OPEN, CLOSE, NUMBER, END, INVALID
} 

fn look_token(s: &mut &str) -> Token { // token remains to be consumed
 *s = (&s).trim();  //discard blanks
 if s.len()== 0 { return Token::END; }
 let c: char = s.chars().nth(0).unwrap();
 if c.is_digit(10) { return Token::NUMBER; }
 match c {
     '+' => return Token::PLUS,
     '*' => return Token::MULT,
     '(' => return Token::OPEN,
     ')' => return Token::CLOSE,
     _ => return Token::INVALID,
 }
}

fn next_char(s: &mut &str) { // consume 1 char
   *s = &s[1..];
}

// Der Parser:
fn number(s: &mut &str) -> Option<Box<Exp>> { // digit ahead, consume digits
  let mut count = 0;  //number of digits

  while count < s.len() && s.chars().nth(count).unwrap().is_digit(10) {
	count += 1;
  }  
  let result:i32 = s[..count].parse().unwrap();
  *s = &s[count..];

  Some(Box::new(Exp::Int { val: result }))
}

fn sum(s: &mut &str)-> Option<Box<Exp>> { // Produkt oder Produkt + Summe
     let result = mult(s);
     if result.is_some() && look_token(s) == Token::INVALID {return error("Ungültiges Zeichen");}
     if look_token(s) != Token::  PLUS  { return result; }
     next_char(s); //skip +
     let right_value = sum(s);
     if result.is_none() || right_value.is_none() {
         return None;
     }
     Some(Box::new(Exp::Plus { left: result.unwrap(), right: right_value.unwrap() } ))
}

 fn mult(s: &mut &str) -> Option<Box<Exp>> { // Wert oder Wert * Produkt
    let result = value(s);
    if result.is_none(){
        return None;
    }
    if look_token(s) != Token::MULT { return result; }
    next_char(s); //skip *
    let right_value = mult(s);
     if right_value.is_none() {
         return None;
     }
     Some(Box::new(Exp::Mult { left: result.unwrap(), right: right_value.unwrap() } ))
}

fn value(s: &mut &str) -> Option<Box<Exp>> {// geklammerter Ausdruck oder Zahl
    match look_token(s) {
        Token::OPEN => {
            next_char(s);// (
                let result = expression(s);
                if result.is_some() {
                  if look_token(s) == Token::INVALID { return error("Ungültiges Zeichen"); }
                  if look_token(s) != Token::CLOSE { return error("Schließende Klammer fehlt"); }
                  next_char(s); // )
                }
                return result;
        }
        Token::NUMBER => return number(s),
        Token::CLOSE => return error("Operand fehlt"),
        Token::INVALID => return error("Ungültiges Zeichen"),
        _ => return error("Doppelter Operator"),
    }
}

fn expression(s: &mut &str) -> Option<Box<Exp>> {
        let token = look_token(s);
        if token == Token::PLUS { next_char(s); } //skip +
        match token {
            Token::END => return error("Unzulässiges Ende"),
            Token::CLOSE => return error("Öffnende Klammer fehlt"),
            Token::MULT => return error("Ungültiges Mal"),
            Token::INVALID => return error("Ungültiges Zeichen"),
            _ => return sum(s),
        }
}

fn error(message: &str)-> Option<Box<Exp>> {
    println!("{}", message);
    return None;
}

pub fn run(input: &str) {
    let mut rest = input; 
    let root = expression(&mut rest);
    let label = "Input: ";
    println!("{0}{1}", label, input);
    if root.is_none() {
        println!("{:->1$}","^", input.len() - rest.len()+ label.len() + 1);
        println!("Result: Fehler");

    }
    else {
        let tree = &root.unwrap();
        println!("Parsed: {0}", show_exp(tree));
        println!("Result: {0}", eval_exp(tree));
    }
}
