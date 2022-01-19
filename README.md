# **Projektarbeit - Parser für arithmetische Ausdrücke**
# Isabel Harms
## Aufgabenstellung:
Als Beispiel für die Arbeitsweise eines Parsers sollen in diesem Projekt einfache arithmetische Ausdrücke analysiert und ausgewertet werden, beschränkt auf die beiden Operatoren "+" und "*"

https://sulzmann.github.io/SoftwareProjekt/schein.html

# **AST**
# Basisklasse Expressions
# Trait Exp
Die Basisklasse für Expressions sollte zunächst durch Vererbung mithilfe von der rust-spezifischen *traits* Mechanik implementiert werden.

Der ```trait Exp``` ist hierbei das Muster für abgeleitete Strukturen, die alle eine ```eval``` Funktion implementieren müssen. So soll mithilfe von Rekursion jeder Expression-Typ seinen eigenen Wert zurückgeben können.

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
Bei dieser Herangehensweise kam es jedoch zu allerlei Problemen, unter anderem mit dem *dynamischen* Speicherbedarf von Expressions.
## Keyword: dyn
```dyn ``` kann als Präfix eines *Traits* verwendet werden und kennzeichet, dass Speicherbedarf erst zur Laufzeit noch ermittelt werden muss.
Da dies aber nicht das einzige Problem war, habe ich mich nach langem Probieren dazu entschieden, die Aufgabenstellung anders zu lösen:
# Enum Exp
Obwohl die Varianten durch ein *enum* nicht erzwungen werden können, die nötigen Funktionen zu implementieren (später durch ein ```match``` in der Funktion gelöst), halte ich es trotzdem für die elegantere Lösung.

Die arithmetischen Ausdrücke wurden begrenzt und enthalten daher nur folgende Elemente:

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
Die ```Error``` Expression soll dazu dienen, einen gefunden Fehler in den AST einzubauen, um ein ```panic!``` und einen folgenden Abbruch des Programmes zu verhindern.So wird ermöglicht, dass nach einem absichtlich eingebauten Fehler noch weitere Tests durchlaufen können.
```
Error {
},
```


# **Ergebnis darstellen**

# Methoden in exp.rs
## show_exp()
Diese Funktion sucht rekursiv die Struktur des Baumes ab, konvertiert die Werte der Endknoten zu Strings und fügt anhand der Verzweigungen und Exp-Typen die richtigen Klammern und Opratoren.
```
Exp::Plus{left, right} => { let s = "(".to_string() + &show_exp(&left) + &"+".to_string() + &show_exp(&right) + &")".to_string();
```
## eval_exp()
Zum Evaluieren des *AST* wird ähnlich vorgegangen, jedoch werden die Werte direkt, von Unten nach Oben, zusammengerechnet.
```
Exp::Plus{left, right} => eval_exp(&left)+eval_exp(&right),
```
# **Tokenizer**

Die verschiedenen Characters, die in den Ausdrücken vorkommen können werden durch folgendes *enum*  beschrieben:
```
enum Token {
    PLUS, MULT, OPEN, CLOSE, NUMBER, END, INVALID
} 
```
Da enums nicht automatisch die Vergleichsoperationen implementieren, erben die **Tokens** von der Funktion ```PartialEq```.


Die Aufgabe des **Tokenizers** ist es lediglich, das nächste Zeichen einzulesen und einer der Varianten zuzuordnen.

Die Methode ```look_token``` übernimmt diese Aufgabe und gibt einen eindeutigen ```Token``` zurück, dessen syntaktische Zulässigkeit vom **Parser** interpretiert werden kann.


# **Parser**

# Grammatik

## Erste Version
Beim ersten Anlauf handelte es sich nicht um eine wirkliche Grammatik. 
Stattdessen wurde nach dem äußersten Operator gesucht und der Eingabe-String wurde in einen rechts und einen links von diesem Operator stehenden Teil eingeteilt. Diese Teile bilden dann ```left``` und ``` right``` von der entsprechenden Variante. Wurde kein äußerstes Plus gefunden, so macht der Parser beim Mal weiter.
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
Exp    -> Sum    | +Sum
Sum    -> Mult   | Mult + Sum
Mult   -> Value  | Value * Mult
Value  -> (Exp)  | Number
Number -> [0..9] | [1..9]*10 + Number
```
Entspricht ein **Token** keinem der vorgegebenen Optionen, so wird ein Fehler ausgegeben.
Der Zustand **Exp** verarbeitet ein ggf. vorhandes unäres Plus. Außerdem findet hier der Übersicht halber eine erste Fehlerüberprüfung statt, um den Code in **Sum** zu vereinfachen.

## Beispiel Anhand von Plus
Zur Vereinfachung gehen wir von einem gültigen Ausdruck aus.

Da in beiden Alternativen der Grammatik **Mult** vorkommt, muss dies immer aufgerufen werden.
```
let result = mult(s);
```
Ist das nächste Zeichen nach dem abgeschlossenen Aufruf nun kein Plus, handelt es sich um Alternative 1: ```Sum -> Mult``` und das Ergebnis kann zurückgegeben werden.
```
if look_token(s) != Token::  PLUS  { return result; }
```
Ist das Zeichen jedoch ein Plus, so handelt sich um Alternative 2: ```Sum -> Mult + Sum```. Nachdem das Plus verbraucht wurde, ergibt sich die linke Seite aus der bereits ausgeführten Multiplikation, für die rechte Seite beginnt die Prozedur von vorne.
```
next_char(s); //skip +
Box::new(Exp::Plus { left: result, right: sum(s) } ) 
```
Mit diesem Prinzip können beliebig viele Summanden aneinander gehängt werden, bei denen es sich jeweils um ein Produkt handelt:

Mult + Mult + Mult + ....

In der Funktion ```mult``` wird equivalent vorgegangen, wobei eine Multiplikation immer aus einer Kette von Ausdrücken besteht, welche aus einer einfachen Zahl oder einem geklammerten Unterausdruck aufgebaut sind.
## Box
Eine *Box* beschreibt eine Referenz zu allokiertem Speicher auf dem *Heap* und wird benötigt, um mithilfe einer bekannten Größe auch rekursive Verweise zu ermöglichen.
Da die Größe unserer **Expressions** zur Zeit des Kompilierens noch nicht bekannt ist und Rust aufgrund seines ungewöhnlichen Memory-Managements sehr streng ist, was unbekannten Speicherbedarf angeht, müssen unsere **Expressions** "geboxt" werden.

## Option
Da es in Rust keine Nullpointer gibt,ermöglichen *Options* eine Funktion, die in anderen Sprachen oft als *nullable* bekannt ist.
Als Option bezeichnet man eine Referenz, welche entweder ein Objekt, oder keines beinhaltet; also einen *optionalen* Wert.
```
pub enum Option<T> {
    None,
    Some(T),
}
```
Dies hat eine Vielzahl von Verwendungsmöglichkeiten, unter anderem für das *Pattern-Matching* oder *Partielle Funktionen*.

Da im Falle eines syntaktischen Fehlers im arithmetischen Ausdruck kein Baum erstellt werden soll, müssen die Funktionen, welche die Grammatik aufbauen die Möglichkeit haben kein Objekt zurückzugeben, um so darzustellen, dass das vorgefundene Zeichen unlogisch ist. Stellt ein Blatt fest, dass mindestens eine Unterkomponente ein ```None``` zurückgegeben hat, so reicht es dies nach oben weiter bis zur Wurzel. Gibt es also einen Syntaxfehler, so gibt es auch keinen *AST*.
```
if result.is_none() || right_value.is_none() {  //left & right
         return None;
    }
```
Die Spezifikation der Art des Fehlers, ergibt sich aus einer weiteren Methode:
## error()
Um eine Information darüber zu erhalten, warum der Parser einen Fehler erkannt hat, wird anstelle der direkten Rückabe eines *None*, die Funktion ```error()``` aufgerufen. Diese nimmt eine Nachricht entgegen, in welcher der Parser eine kurzes Statement zum Abbruch übergeben kann.
```
fn error(message: &str)-> Option<Box<Exp>> {
    println!("{}", message);
    return None;
}
```

# Tests
Die Tests laufen über das file main.rs. Dort habe ich 2 Arrays mit arithmetischen Ausdrücken und jeweiligen Lösungen hinterlegt, welche in einer einfachen Schleife durchlaufen und evaluiert werden.
Für jeden Test wird eine Nummer und das erwartete Ergebnis ausgeben, sowie die ```run``` methode von exp.rs aufgerufen.

## run()

Zunächst wird, egal ob der Parser einen Baum erzeugt hat oder nicht, der Original-Text gedruckt.

Nun wird die Methode ```expression```, die als Startzustand unserer Grammatik dient, mit einem *mutable* Pointer auf unseren Ausdruck aufgerufen. Der Parser erzeugt einen Baum, oder im Fehlerfall ein *None*.

Im Falle eines gültigen *AST* können unsere Untermethoden aufgerufen werden.
```
let tree = &root.unwrap();
println!("Parsed: {0}", show_exp(tree));
println!("Result: {0}", eval_exp(tree));
```
Gibt es jedoch keinen *AST*, also gilt: ```root.is_none()```, so wird stattdessen unterhalb des Input-Strings ein Pfeil ausgegeben, der mithilfe der Differenz zwischen Rest-, und Original-Text die Stelle des Fehlers markiert.
```
println!("{:->1$}","^", input.len() - rest.len()+ label.len() + 1);
println!("Result: Fehler");
```
Beispielsweise ergibt sich so für einen unserer Tests folgende Ausgabe:
```
Test No.7:
Schließende Klammer fehlt
Input:  12+(23*4
----------------^
Result: Fehler
Expected: Fehler
```
# **Quellen**

https://doc.rust-lang.org/book/title-page.html

https://www.youtube.com/watch?v=zF34dRivLOw&t=4102s

https://doc.rust-lang.org/stable/std/

