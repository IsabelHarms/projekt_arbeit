pub trait Exp // "muster"
{
    fn eval(&self) -> i32; // Expressions müssen eval implementieren
}

pub struct Plus<T:Exp> // Plus erfüllt das Exp-Muster
{
    pub left: T,
    pub right: T
}

struct PlusN<T:Exp> { //bisher nicht benutzt
  operands : Vec<T> 
}

pub struct Mal<T:Exp> // Mal erfüllt das Exp-Muster
{
    pub left: T,
    pub right: T
}

pub struct Int { pub val: i32 } // Int erfüllt das Exp-Muster


// die Implementierungen von eval: 
impl Exp for Int
{
    fn eval(&self) -> i32
    {
        return self.val
    }
}

impl<T:Exp> Exp for Plus<T>
{
    fn eval(&self) -> i32
    {
      return self.left.eval() + self.right.eval()
    }
}

impl<T:Exp> Exp for Mal<T>
{
    fn eval(&self) -> i32
    {
      return self.left.eval() * self.right.eval()
    }
}

pub fn run()
{
  let input = "5+(2+3) * (4+1)";
  let mut rest = input; 
  let expression:dyn Exp = ausdruck(&mut rest);
  let result:i32 = expression.eval();
  println!("{0} = {1}", input, result);
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
     *s = (&s).trim();
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
fn zahl(s: &mut &str) -> Int // digit ahead, consume digits
{
  let mut count = 0;

  while count < s.len() && s.chars().nth(count).unwrap().is_digit(10)
  {
	count += 1;
  }  
  let result:i32 = s[..count].parse().unwrap();
  *s = &s[count..];
  
  println!("Zahl {}", result);
  println!("Rest '{}'", s);
  Int { val: result }
}

fn summe(s: &mut &str)-> dyn Exp // Produkt oder Produkt + Summe
{
     let result = produkt(&mut s);
     if look_token(&mut s) != PLUS  { return result; }
     next_char(&mut s);
     Plus { left: result, right: summe(&mut s) } // return new struct object
}

 fn produkt(s: &mut &str) -> dyn Exp // Wert oder Wert * Produkt
{
    let result = wert(&mut s);
    if look_token(&mut s) != MAL { return result; }
    next_char(&mut s);
    Mal { left: result, right: produkt(&mut s) } // return new struct object
}

fn wert(s: &mut &str) -> dyn Exp // geklammerter Ausdruck oder Zahl
{
   if look_token(&mut s)== KLAUF
   {
      next_char(&mut s);// (
      let result = ausdruck(&mut s);
      if look_token(&mut s) != KLZU { fehler("schließende Klammer fehlt"); }
      next_char(&mut s); // )
      return result;
   }
   if look_token(&mut s) == ZIFFER { return zahl(&mut s); }

   fehler("Syntaxfehler");
 }
 
 fn ausdruck(s: &mut &str) -> dyn Exp // Summe oder leer oder (Wert)
 {
 
     let token = look_token(&mut s);
     if token == ENDE { fehler("leerer Ausdruck"); }
     if token == UNGÜLTIG { fehler("unzulässige(s) Zeichen"); }
     if token == KLZU { fehler("falsche Klammerung, fehlt Klammer auf?"); }
     if token ==   MAL { fehler("Syntaxfehler, fehlt ein Faktor?"); }
     if token ==  PLUS { next_char(&mut s); }

     summe(&mut s)
 }
 
// einfache Version: stopp beim 1. Syntaxfehler
 
 fn fehler(meldung: &str) -> ! // never returns
 {
     panic!("Fehler: {}", meldung); 
 }
