# **Projektarbeit Parser für arithemtische Ausdrücke**
# Isabel Harms
## Aufgabenstellung:
https://sulzmann.github.io/SoftwareProjekt/schein.html

# **AST**
# Basisklasse Expressions
## Trait Exp
Die Basisklasse für Expressions sollte zunächst durch eine Art Vererbung mithilfe von der rust-spezifischen ```trait``` Mechanik implementiert werden.

Der ```trait Exp``` ist hierbei das Muster für abgeleitete Strukturen, die alle eine ```eval``` Funktion implementieren sollen. So soll mithilfe von Rekursion jeder Expression Typ seinen eigenen Wert zurückgeben können.

```
pub trait Exp { 
    fn eval(&self) -> i32; 
} 

pub struct Plus<T:Exp> { 
    pub left: T, 
    pub right: T 
}

impl<T:Exp> Exp for Plus<T> { 
    fn eval(&self) -> i32 { 
      return self.left.eval() + self.right.eval() 
    } 
} 
```
Bei dieser Herangehensweise kam es jedoch zu allerlei Problemen, unteranderem ->
## Keyword: dyn
Daher habe ich mich nach langem Probieren dazu entschieden das Problem anders zu lösen:
# Enum Exp
Obwohl die Varianten durch ein ```enum``` nicht erzwungen werden können die nötigen Funktionen zu implementieren (später durch ein ```match``` in der Funktion gelöst), halte ich es trotzdem für eine effizientere Lösung.

Die Arithmetischen Ausdrücke wurden begrenzt und enthalten daher nur folgende Elemente:

## PlusExp & MultExp
Plus und Mal verbinden  jeweils 2 untergeordnete **Expressions** und bauen somit die Verzweigungen des **AST* auf.
```
Plus {                              //         Plus
        left: Box<Exp>,             //        /    \
        right: Box<Exp>,            //     left    right
},

Mult {                              //         Mult
        left: Box<Exp>,             //        /    \
        right: Box<Exp>,            //     left    right
},
```

## IntExp
Die Endknoten des **AST** bestehen aus Zahlen, welche nur ihren eigenen Wert enthalten.
```
Int {                               //         Plus
        val: i32,                   //        /    \                   
},                                  //      Int    Mult
```
## ErrorExp
Die ```Error``` Expression soll dazu dienen, einen gefunden Fehler in den AST einzubauen, um ein ```panic!``` und einen folgenden Abbruch des Programmes zu verhindern,um somit zu ermöglichen, dass z.B. nach einem absichtlich eingebauten Fehler noch weitere Tests durchlaufen zu können. Außerdem ist es so möglich die Stelle und den Grund des Fehlers nach Oben durchzureichen.
```
```
# Option

# Box
Eine **Box** beschreibt eine Referenz zu allokiertem Speicher auf dem **Heap** und wird benötigt um mithilfe einer bekannten statischen Größe auch bei rekursiven Aufrufen eine ...sicherzustellen.
Da die größe unserer **Expressions** zur Zeit des Kompilierens noch nicht bekannt ist und Rust sehr streng ist, was ... angeht müssen alle **Expression** geboxt werden.

# **Tokenizer**

Die verschiedenen Zeichen, die in den Ausdrücken vorkommen können werden durch folgendes **enum** beschrieben:
```
enum Token {
    PLUS, MULT, OPEN, CLOSE, NUMBER, END, INVALID
} 
```
Da enums nicht automatisch die Vergleichsoperationen implementieren erben die **Tokens** von der Funktion ```PartialEq```.


Die Aufgabe des **Tokenizers** ist es lediglich, das nächte/erste Zeichen einzulesen und einem der Varianten zuzuordnen.

Die Methode ```look_token``` soll diese Aufgabe übernehmen und einen eindeutigen ```Token``` zurückgeben, dessen logischer Zusammenhang vom **Parser** interpretiert werden kann.


# **Parser**

# Grammatik

## Erste Version
Beim ersten Anlauf handelt es sich nicht um eine wirkliche Grammatik. 
Stattdessen wird nach dem Äußersten Operator gesucht und der Eingabe String wird in einen rechten und einen linken von diesem Operator stehenden Teil eingeteilt. Diese Teile bilden dann ```left``` und ``` right``` von der entsprechenden Variante. Wurde kein äußerstes Plus gefunden, so macht der Parser beim Mal weiter.
Da der Ausdruck hierbei jedoch immer wieder von vorne durchsucht werden muss ist dies suboptimal für die Laufzeit und die Idee wurde verworfen.
```
fn outer_plus(s: &str)-> usize
{
    let mut depth: i16 = 0;
    for i in 0..s.len()-1
    {
        let mut depth: i16 = 0;
        let c = s.chars().nth(i);
        match c
        {
            Some('(') => depth=depth+1,
            Some(')') => depth=depth-1,
            Some('+') => if (depth == 0) {return i;},
            other => depth=depth,  //ugly
            Some('+') if (depth == 0) => return i,
            _other => depth=depth,  //ugly

        }
        //Finde ein Plus auf Klammerungsebene 0
        //Klammernzählen
    }
    //if depth is not 0 here, the given expression was invalid
    s.len() //No outer plus found
}
```
## Finale Version
```
Exp    -> Sum
Sum    -> Mult   | Mult + Sum
Mult   -> Value  | Value * Mult
Value  -> (Exp)  | Number
Number -> [0..9] | [1..9]*10 + Number
```
Entspricht ein **Token** keinem der vorgegebenen Optionen, so wird ein Fehler ausgegeben.
Der Zustand **Exp** scheint überflüssig, hier findet jedoch der Übersicht halber eine erste Fehlerüberprüfung statt, um den Code in **Sum** zu vereinfachen.
## Beispiel Anhand von Plus
```
```
# **Evaluate Results**
# show_exp
```
```
# eval_exp
```
Exp::Plus{left, right} => { let s = "(".to_string() + &show_exp(&left) + &"+".to_string() + &show_exp(&right) + &")".to_string();
```
# Tests
```
```

# **Quellen**

https://doc.rust-lang.org/book/title-page.html

https://www.youtube.com/watch?v=zF34dRivLOw&t=4102s

https://doc.rust-lang.org/stable/std/

