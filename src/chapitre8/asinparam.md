# Les closures passées en paramètres

Alors que Rust se charge de choisir, pour les closures, la manière de capturer les variables sans forcer le typage, lorsque c'est possible, cette ambiguïté n'est pas permise au sein des fonctions. Lorsqu'une closure est passée en paramètre à une fonction, le type de ses paramètres ainsi que celui de sa valeur de retour doivent être précisés en utilisant des traits. Dans l'ordre du plus restrictif au plus « laxiste » :

1. `Fn` : La closure capture par référence (`&T`) ;
2. `FnMut` : La closure capture par référence mutable (`&mut T`) ;
3. `FnOnce` : La closure capture par valeur (`T`).

En se fiant au contexte, le compilateur va capturer les variables en privilégiant le « régime » le moins restrictif possible.

Par exemple, prenez un paramètre typé avec le trait `FnOnce`. Cela signifie que la closure *peut* capturer ses variables par référence `&T`, référence mutable `&mut T`, ou valeur `T` mais le compilateur reste encore le seul juge quant au régime à adopter, en fonction du contexte.

C'est pourquoi si un transfert (`move`) est possible alors n'importe quel type d'emprunts devrait être possible, notez que l'inverse n'est pas vrai. Si le paramètre est typé `Fn` alors les captures par référence mutable `&mut T` ou par valeur `T` ne sont pas permises.

Dans l'exemple suivant, essayez de modifier le type de capture (i.e. `Fn`, `FnMut` et `FnOnce`) pour voir ce qu'il se passe :

{{#playpen source/asinparamsource0.rs}}

## Voir aussi

La fonction [std::mem::drop][drop] et les traits [Fn][Fn], [FnMut][FnMut] et [FnOnce][FnOnce].

[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
[Fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[FnMut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[FnOnce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
