# **Projektarbeit Parser für arithemtische Ausdrücke**
# Isabel Harms
## Aufgabenstellung:
https://sulzmann.github.io/SoftwareProjekt/schein.html

# **AST**
# Basisklasse Expressions
# Trait Exp
Die Basisklasse für Expressions sollte zunächst durch eine Art Vererbung mithilfe von der rust-spezifischen *trait* Mechanik implementiert werden.

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
Bei dieser Herangehensweise kam es jedoch zu allerlei Problemen, unteranderem mit dem unbekannten Speicherbedarf von Expressions.
## Keyword: dyn
```dyn ``` kann als Präfix eines *Traits* verwendet werden und ist ein Kennzeichen, dass speicherbedarf erst zur Laufzeit noch ermittelt werden muss.
Da dies aber nicht das einzige Problem war, habe ich mich nach langem Probieren dazu entschieden die Aufgabenstellung anders zu lösen:
# Enum Exp
Obwohl die Varianten durch ein *enum* nicht gezwungen werden können die nötigen Funktionen zu implementieren (später durch ein ```match``` in der Funktion gelöst), halte ich es trotzdem für eine effizientere Lösung.

Die Arithmetischen Ausdrücke wurden begrenzt und enthalten daher nur folgende Elemente:

## PlusExp & MultExp
Plus und Mal verbinden  jeweils 2 untergeordnete **Expressions** und bauen somit die Verzweigungen des *AST* auf.
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
Die Endknoten des *AST* bestehen aus Zahlen, welche nur ihren eigenen Wert enthalten.
```
Int {                               //         Plus
        val: i32,                   //        /    \                   
},                                  //      Int    Mult
```
## ErrorExp
Die ```Error``` Expression soll dazu dienen, einen gefunden Fehler in den AST einzubauen, um ein ```panic!``` und einen folgenden Abbruch des Programmes zu verhindern,um somit zu ermöglichen, dass nach einem absichtlich eingebauten Fehler noch weitere Tests durchlaufen zu können.
```
Error {
},
```
## Option
Da es in Rust keine Nullpointer gibt,ermöglichen *Options* eine Funktion, die in anderen Sprachen of als *nullable* bekannt ist.
Als Option bezeichnet man eine Referenz, welche entweder ein Objekt mit einem Wert, oder keines Beinhaltet; also einen *optionalen* Wert.
```
pub enum Option<T> {
    None,
    Some(T),
}
```
Dies hat eine vielzahl von Verwendungsmöglichkeiten, unter anderem für das *Pattern-Matching* oder *Partielle Funktionen*.
Zwischenzeitlich wurde diese Funktionalität auch in meinem Code verwendet, wurde jedoch nach einiger Zeit durch **Boxen** ersetzt.
Dennoch wollte ich sie hier integrieren, da sie zur Perfektionierung dieses Programms hilfreich wären um ein weiteres match zu verwenden und ein besseres Einbauen der Fehler in den Parser ermöglichen würden.

## Box
Eine *Box* beschreibt eine Referenz zu allokiertem Speicher auf dem *Heap* und wird benötigt um mithilfe einer bekannten statischen Größe auch rekursives Aufrufen zu ermöglichen.
Da die größe unserer **Expressions** zur Zeit des Kompilierens noch nicht bekannt ist und Rust Aufgrund seines alternativen Memory-Managements sehr streng ist, was unbekannten Speicherbedarf angeht, müssen unsere **Expression** geboxt werden.

# **Tokenizer**

Die verschiedenen Zeichen, die in den Ausdrücken vorkommen können werden durch folgendes *enum*  beschrieben:
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
Zur Vereinfachung gehen wir von einem gültigen Asudruck aus.

Da in beiden Alternativen der Grammatik **Mult** vorkommt, muss dies immer aufgerufen werden.
```
let result = mult(s);
```
Ist das nächste Zeichen nach dem abgeschlossenen Aufruf nun kein Plus handelt es sich um Alternative 1: ```Sum -> Mult``` und das Ergebnis kann zurückgegeben werden.
```
if look_token(s) != Token::  PLUS  { return result; }
```
Ist das Zeichen jedoch ein Plus, so handelt sich um Alternative 2: ```Sum -> Mult + Sum```. Nachdem das Plus gelöscht wurde, ergibt sich die linke Seite aus der bereits ausgeführten Multiplikation, für die rechte Seite beginnt die Prozedur von vorne.
```
next_char(s); //skip +
Box::new(Exp::Plus { left: result, right: sum(s) } ) 
```
Mit diesem Prinzip können beliebig viele Summanden aneinander gehängt werden, bei denen es sich jeweils um ein Produkt handelt:

Mult + Mult + Mult + ....

In der Funktion ```mult``` wird equivalent vorgegangen, wobei eine Multiplikation immer aus einem Ausdruck besteht, welcher aus einer einfachen Zahl oder einem geklammerten Unterausdruck aufgebaut ist.

# **Ergebnis darstellen**
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

