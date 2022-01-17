
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
    Error {
    },
}


// Show for expressions
fn show_exp(x : &Exp) -> String {
    match x {
        Exp::Int{val} => { return val.to_string(); }
        Exp::Plus{left, right} => { let s = "(".to_string() + &show_exp(&left)
                                             + &"+".to_string() + &show_exp(&right) + &")".to_string();
                                    return s; }
        Exp::Mult{left, right} => { let s = "(".to_string() + &show_exp(&left)
                                            + &"*".to_string() + &show_exp(&right) + &")".to_string();
                                    return s; }
        Exp::Error{} => {return "Fehler".to_string();}
    }
}

// Evaluation for expressions
fn eval_exp(x: &Exp) -> i32 {
    match x {
      Exp::Int{val} => *val,
      Exp::Plus{left, right} => eval_exp(&left)+eval_exp(&right),
      Exp::Mult{left, right} => eval_exp(&left)*eval_exp(&right),
      Exp::Error{} => -1000, //Urgh
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
fn number(s: &mut &str) -> Box<Exp> { // digit ahead, consume digits
  let mut count = 0;  //number of digits

  while count < s.len() && s.chars().nth(count).unwrap().is_digit(10) {
	count += 1;
  }  
  let result:i32 = s[..count].parse().unwrap();
  *s = &s[count..];

  Box::new(Exp::Int { val: result })
}

fn sum(s: &mut &str)-> Box<Exp> { // Produkt oder Produkt + Summe
     let result = mult(s);
     if look_token(s) == Token::INVALID {return error("Unzulässiges Zeichen");}
     if look_token(s) != Token::  PLUS  { return result; }
     next_char(s); //skip +
     Box::new(Exp::Plus { left: result, right: sum(s) } )
}

 fn mult(s: &mut &str) -> Box<Exp> { // Wert oder Wert * Produkt
    let result = value(s);
    if look_token(s) != Token::MULT { return result; }
    next_char(s); //skip *
    Box::new(Exp::Mult { left: result, right: mult(s) }) // return new struct object
}

fn value(s: &mut &str) -> Box<Exp> {// geklammerter Ausdruck oder Zahl
   if look_token(s)== Token::OPEN {
      next_char(s);// (
      let result = expression(s);
      if look_token(s) != Token::CLOSE { return error("falsche Klammerung"); }
      next_char(s); // )
      return result;
   }
   if look_token(s) == Token::NUMBER { return number(s); }

   error("Syntaxfehler")
 }

fn expression(s: &mut &str) -> Box<Exp> {
        let token = look_token(s);

        if token == Token::END { return error("leerer Ausdruck"); }
        if token == Token::CLOSE { return error("falsche Klammerung"); }
        if token == Token::MULT { return error("ungültiges *"); }
        if token == Token::INVALID {return error("Unzulässiges Zeichen");}
        if token == Token::PLUS { next_char(s); }

        sum(s)
}

pub fn run(input: &str) {
    let mut rest = input; 
    let root = expression(&mut rest);
    println!("Input:  {0}", input);
    println!("Parsed: {0}", show_exp(&root));
    println!("Result: {0}", eval_exp(&root));
}

fn error(message: &str) -> Box<Exp> {
     println!("{}",message);
     Box::new(Exp::Error{}) 
 }
