# Les expressions

Un programme écrit en Rust est (principalement) composé d'une série de déclarations :

{{#playpen source/exprsource0.rs}}

Il y a plusieurs sortes de déclarations en Rust. Les deux plus communes sont les assignations et les expressions suivies par un point-virgule « ; » :

{{#playpen source/exprsource1.rs}}

Les blocs sont également des expressions, donc ils peuvent être utilisés comme `r-value` dans les assignations. La dernière expression dans le bloc sera assignée à la `l-value`. Notez toutefois que si la dernière expression du bloc se termine par un point-virgule « ; », la valeur de renvoi sera `()`.

{{#playpen source/exprsource2.rs}}
