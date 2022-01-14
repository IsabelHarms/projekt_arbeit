
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
    }
}
fn eval_exp(x: &Exp) -> i32
{
  match x
  {
    Exp::Int{val} => *val,
    Exp::Plus{left, right} => eval_exp(&left)+eval_exp(&right),
    Exp::Mult{left, right} => eval_exp(&left)*eval_exp(&right),
  }
}

// enum doesn't allow comparison using == operator, so:
const PLUS:i8 = 1; 
const MAL:i8 = 2;
const KLAUF:i8 = 3;
const KLZU:i8 = 4;
const ZIFFER:i8 = 5;
const ENDE:i8 = 6;
const UNGÜLTIG:i8 =0; 

// der Tokenizer:
fn look_token(s: &mut &str) -> i8 // token remains to be consumed
{
     *s = (&s).trim();  //discard blanks
     if s.len()== 0 { return ENDE; }
     let c: char = s.chars().nth(0).unwrap();
 
 if c.is_digit(10) { return ZIFFER; }
 if c == '+' { return PLUS; }
 if c == '*' { return MAL;  }
 if c == '(' { return KLAUF;}
 if c == ')' { return KLZU; }

 UNGÜLTIG
}

fn next_char(s: &mut &str) // consume 1 char
{
   *s = &s[1..];
}
// Der Parser:

fn zahl(s: &mut &str) -> Box<Exp> // digit ahead, consume digits
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

fn summe(s: &mut &str)-> Box<Exp> // Produkt oder Produkt + Summe
{
     let result = produkt(s);
     if look_token(s) != PLUS  { return result; }
     next_char(s);
     Box::new(Exp::Plus { left: result, right: summe(s) } )// return new struct object
}

 fn produkt(s: &mut &str) -> Box<Exp> // Wert oder Wert * Produkt
{
    let result = wert(s);
    if look_token(s) != MAL { return result; }
    next_char(s);
    Box::new(Exp::Mult { left: result, right: produkt(s) }) // return new struct object
}

fn wert(s: &mut &str) -> Box<Exp> // geklammerter Ausdruck oder Zahl
{
   if look_token(s)== KLAUF
   {
      next_char(s);// (
      let result = ausdruck(s);
      if look_token(s) != KLZU { fehler("schließende Klammer fehlt"); }
      next_char(s); // )
      return result;
   }
   if look_token(s) == ZIFFER { return zahl(s); }

   fehler("Syntaxfehler");
 }

// Basic structure of parse functions.
// Details are missing!
fn ausdruck(s: &mut &str) -> Box<Exp> {
    //if *s == "" {
    //    return None;
    //} else {
        let token = look_token(s);
        if token == ENDE { fehler("leerer Ausdruck"); }
        if token == KLZU { fehler("falsche Klammerung, fehlt Klammer auf?"); }
        if token ==  MAL { fehler("Syntaxfehler, fehlt ein Faktor?"); }
        if token == PLUS { next_char(s); }
   
        summe(s)
    //}

}

pub fn run() {
    let input = "(2+3) * (1+4)+5 + 8*2";
    let mut rest = input; 
    let root = ausdruck(&mut rest);
    //prüfen ob root none ist
    println!("Input:  {0}", input);
    println!("Parsed: {0}", show_exp(&root));
    println!("Result: {0}", eval_exp(&root));
}

fn fehler(meldung: &str) -> ! // never returns
 {
     panic!("Fehler: {}", meldung); 
 }

 //TODOs
 //consts -> enums: derive PartialEqual
 //ifs -> matches
 //fehler: don't panic
 //tests
