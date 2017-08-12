# Les primitifs

Le langage Rust offre une grande variété de primitifs. Liste non-exhaustive :


* Les entiers signés : `i8`, `i16`, `i32`, `i64` et `isize` (dépend de l'architecture de la machine) ;
* Les entiers non-signés : `u8`, `u16`, `u32`, `u64`, `usize` (dépend de l'architecture de la machine) ;
* Les réels : `f32`, `f64`;
* Les caractères (Unicode) : `‘a'`, `‘α'`, `‘∞'`. Codés sur 4 octets;
* Les booléens : `true` ou `false`;
* L'absence de type `()`, qui n'engendre qu'une seule valeur : `()`;
* Les tableaux : `[1, 2, 3]`;
* Les tuples : `(1, true)`.

Le type des variables peut toujours être spécifié. Les nombres peuvent également être typés grâce à un suffixe, ou par défaut (laissant le compilateur les typer). Les entiers, par défaut, sont typés `i32` tandis les réels sont typés `f64`.

{{#playpen source/primitifssource0.rs}}

## Voir aussi

[La bibliothèque standard][std].

[std]: https://doc.rust-lang.org/std/
