# Fonctions passées en paramètres

Les closures peuvent être soumises en entrée aux fonctions, mais vous pourriez vous demander si nous pouvons faire de même avec d'autres fonctions. C'est le cas ! Si vous déclarez une fonction qui prend une closure en paramètre alors n'importe quelle fonction implémentant les traits requis peut être passée en paramètre.

{{#playpen source/inputfnsource0.rs}}

## Voir aussi

[Fn][Fn], [FnMut][FnMut] et [FnOnce][FnOnce].

[Fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[FnMut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[FnOnce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
