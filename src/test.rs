static mut S:&str = "5+(2+3) * (4+1)";

pub fn run()
{
unsafe
{
  println!("{0} = {1}", S, ausdruck());
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


fn look_token() -> i8 // token remains to be consumed
{ unsafe // using global string pointer
 {
     S = S.trim();
     if S.len()== 0 { return ENDE; }
     let c: char = S.chars().nth(0).unwrap();
 
 if c.is_digit(10) { return ZIFFER; }
 if c == '+' { return PLUS; }
 if c == '*' { return MAL;  }
 if c == '(' { return KLAUF;}
 if c == ')' { return KLZU; }
 }
 UNGÜLTIG
}

fn next_char() // consume 1 char
{
  unsafe { S = &S[1..]; }
}

fn zahl() -> i32 // digit ahead, consume digits
{
  let mut count = 0;
  unsafe
 {
  while count < S.len() && S.chars().nth(count).unwrap().is_digit(10)
  {
count += 1;
  }  
  let result:i32 = S[..count].parse().unwrap();
  S = &S[count+1..];
 
 result
 }
}

fn summe()->i32 // Produkt + Produkt + ... Produkt
{
     let mut result = 0;
     loop
     {
    result += produkt();
    if look_token() != PLUS {  break; }
        next_char();
     }
     result
}

 fn produkt() -> i32 // Wert * Wert * ... Wert
 {
    let mut result = 1;
    loop
    {
   result *= wert();
     if look_token() != MAL { break; }
     next_char();
     }
     result
 }

  fn wert() -> i32 // geklammerter Ausdruck oder Zahl
  {
   if look_token() == KLAUF
   {
      next_char();// (
      let result = ausdruck();
      if look_token() != KLZU { fehler("schließende Klammer fehlt"); }
      next_char(); // )
      return result;
   }
   if look_token() == ZIFFER { return zahl(); }

   fehler("Syntaxfehler");
 }
 
 fn ausdruck() -> i32 // Summe oder leer oder (Wert)
 {
     let token = look_token();
     if token == ENDE { fehler("leerer Ausdruck"); }
     if token == UNGÜLTIG { fehler("unzulässiges Zeichen"); }
     if token == KLZU { fehler("falsche Klammerung, fehlt Klammer auf?"); }
     if token ==   MAL { fehler("Syntaxfehler, fehlt ein Faktor?"); }
     if token ==  PLUS { next_char(); }

     summe()
 }
 
 fn fehler(meldung: &str) -> ! // never returns
 {
     panic!("Fehler: {}", meldung);
 }