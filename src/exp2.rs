
/* AST for simple expression language and some basic functionality */

enum Exp {
    Int {
        val: i32,
    },
    Plus {
        left: Box<Exp>,  // Box = heap allocated necessary due to recursive definition
        right: Box<Exp>,
    },
    Mult {
        left: Box<Exp>,  // Box = heap allocated necessary due to recursive definition
        right: Box<Exp>,
    },
    Error {
        error: str,
    }
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
        Exp::Error{error} => {return "".to_string();}
    }
}

fn eval_exp(x: &Exp) -> i32
{
  match x
  {
    Exp::Int{val} => *val,
    Exp::Plus{left, right} => eval_exp(&left)+eval_exp(&right),
    Exp::Mult{left, right} => eval_exp(&left)*eval_exp(&right),
    Exp::Error{error} => error(""),
  }
}

// der Tokenizer:
#[derive(PartialEq)]
enum Token {
    PLUS, MULT, OPEN, CLOSE, NUMBER, END, INVALID
} 

fn look_token(s: &mut &str) -> Token // token remains to be consumed
{
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

fn next_char(s: &mut &str) // consume 1 char
{
   *s = &s[1..];
}
// Der Parser:

fn number(s: &mut &str) -> Box<Exp> // digit ahead, consume digits
{
  let mut count = 0;  //number of digits

  while count < s.len() && s.chars().nth(count).unwrap().is_digit(10)
  {
	count += 1;
  }  
  let result:i32 = s[..count].parse().unwrap();  //TODO: Overflow
  *s = &s[count..];
  
  //println!("Zahl {}", result);
  //println!("Rest '{}'", s);
  Box::new(Exp::Int { val: result })
}

fn sum(s: &mut &str)-> Box<Exp> // Produkt oder Produkt + Summe
{
     let result = mult(s);
     if look_token(s) != Token::  PLUS  { return result; }
     next_char(s);
     Box::new(Exp::Plus { left: result, right: sum(s) } )// return new struct object
}

 fn mult(s: &mut &str) -> Box<Exp> // Wert oder Wert * Produkt
{
    let result = value(s);
    if look_token(s) != Token::MULT { return result; }
    next_char(s);
    Box::new(Exp::Mult { left: result, right: mult(s) }) // return new struct object
}

fn value(s: &mut &str) -> Box<Exp> // geklammerter Ausdruck oder Zahl
{
   if look_token(s)== Token::OPEN
   {
      next_char(s);// (
      let result = expression(s);
      if look_token(s) != Token::CLOSE { error("schließende Klammer fehlt"); }
      next_char(s); // )
      return result;
   }
   if look_token(s) == Token::NUMBER { return number(s); }

   error("Syntaxfehler");
 }

fn expression(s: &mut &str) -> Box<Exp> {
        let token = look_token(s);
        match token {
            Token::END => error("leerer Ausdruck"),
            Token::CLOSE => error("falsche Klammerung, fehlt Klammer auf?"),
            Token::MULT => error("Syntaxfehler, fehlt ein Faktor?"),
            Token::PLUS => next_char(s),
            _ => (),
        }

        sum(s)
}

pub fn run(input: &str) {
    //let input = "(2+3) * (1+4)+5 + 8*2";
    let mut rest = input; 
    let root = expression(&mut rest);
    //prüfen ob root none ist
    println!("Input:  {0}", input);
    println!("Parsed: {0}", show_exp(&root));
    println!("Result: {0}", eval_exp(&root));
}

fn error(meldung: &str) -> Box<Exp> // never returns
 {
     Box::new(Exp::Error{error: meldung}) 
 }

 //TODOs
 //error: don't panic
