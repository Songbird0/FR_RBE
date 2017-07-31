# macro_rules!

Rust fournit un puissant système de macros qui permet la [métaprogrammation](https://fr.wikipedia.org/wiki/M%C3%A9taprogrammation). Comme vous l'avez vu dans les chapitres précédents, les macros ressemblent aux fonctions, excepté que leurs identificateurs se terminent par un point d'exclamation `!`, mais au lieu de générer un appel de fonction, les macros sont étendues dans le code source et compilées avec le reste du programme.

Il est possible de créer une macro en utilisant la macro `macro_rules!`.

{{#playpen source/systememacrosource0.rs}}
