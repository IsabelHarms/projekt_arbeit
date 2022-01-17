# **Projektarbeit Parser für arithemtische Ausdrücke**
# Isabel Harms
## Aufgabenstellung:
https://sulzmann.github.io/SoftwareProjekt/schein.html

# **AST**
# Basisklasse Expressions
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
# Keyword: dyn
Daher habe ich mich nach langem Probieren dazu entschieden das Problem anders zu lösen:
# **Enum Exp**
Obwohl die Varianten durch ein ```enum``` nicht erzwungen werden können die nötigen Funktionen zu implementieren (später durch ein ```match``` gelöst), halte ich es trotzdem für eine effizientere Lösung.
# Abgeleitete Varianten für die einzelnen Fälle
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

# Boxen
Eine **Box** beschreibt eine Referenz zu allokiertem Speicher auf dem **Heap** und wird benötigt um mithilfe einer bekannten statischen Größe auch bei rekursiven Aufrufen eine ...sicherzustellen.
Da die größe unserer **Expressions** zur Zeit des Kompilierens noch nicht bekannt ist und Rust sehr streng ist, was ... angeht müssen alle **Expression** geboxt werden.

# **Tokenizer**

Die Aufgabe des **Tokenizers** ist es lediglich, das nächte/erste Zeichen einzulesen und einem der folgenden Varianten zuzuordnen.
```
enum Token {
    PLUS, MULT, OPEN, CLOSE, NUMBER, END, INVALID
} 
```
Da enums nicht automatisch die Vergleichsoperationen implementieren erben die **Tokens** von der Funktion ```PartialEq```.

Die Methode ```look_token``` soll diese Aufgabe übernehmen und einen eindeutigen ```Token``` zurückgeben, dessen logischer Zusammenhang vom **Parser** interpretiert werden soll.


# **Parser**

# Grammatik
```
Exp    -> Sum
Sum    -> Mult   | Mult + Sum
Mult   -> Value  | Value * Mult
Value  -> (Exp)  | Number
Number -> [0..9] | [1..9]*10 + Number
```
## Beispiel Anhand von Plus
```
```
# **Evaluate Results**
# show_exp
```
```
# eval_exp
```
```
# Tests
```
```

# **Quellen**

https://doc.rust-lang.org/book/title-page.html

https://www.youtube.com/watch?v=zF34dRivLOw&t=4102s

https://doc.rust-lang.org/std/keyword.dyn.html

