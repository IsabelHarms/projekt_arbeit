Quellen bisher:
https://doc.rust-lang.org/book/title-page.html
https://www.youtube.com/watch?v=zF34dRivLOw&t=4102s

Idee zur Realisierung eines Syntax Baumes ohne Klassen in Rust:

- Für jede syntaktische Einheit eine eigene Funktion
- Entsprechende Funktion kann Codeteile von links abschneiden (Code als langer string?)
- bsp: fn block {curly-open(); instruction(); curly-close();}
- curly-open erwartet "{" (Leerzeichen?)
- Quelltext wird auf bestimmte Anfänge getestet und entsprechende Funktion wird aufgerufen (Switch?) (welche Möglichkeiten für Anfänge gibt es?)
- Operatoren-Liste
- "verbrauchte" Codestücke müssen abgeschnitten werden! (Restteile evtl. als slice?)
