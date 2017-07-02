# Renvoyer une closure

Les closures peuvent être passées en paramètre à une fonction, donc les renvoyer devrait être possible. Cependant, renvoyer un « type » de closure est problématique car, actuellement, Rust ne supporte le renvoi que de *types concrets* (i.e. non-génériques). Le type anonyme d'une closure est, par définition, inconnu donc le renvoi d'une closure ne peut être fait qu'en rendant son type concret.

Les traits destinés à valider le renvoi d'une closure sont quelque peu différents :


* `Fn` : pas de changements pour ce trait ;
* `FnMut` : pas de changements pour ce trait ;
* `FnOnce` : Différentes choses entrent en jeu ici, donc le type `FnBox` doit être utilisé à la place de `FnOnce`. Notez toutefois que `FnBox` est taggé **instable** et que des modifications pourraient être apportées dans le futur.

En dehors de cela, le mot-clé `move` doit être utilisé, indiquant que toutes les captures se feront par valeur pour la closure courante. Il est nécessaire d'utiliser `move` car aucune capture par référence ne pourrait être libérée aussitôt la fonction terminée, laissant des références invalides dans la closure.

{{#playpen source/outputclosuresource0.rs}}

## Voir aussi

[Box](../chapitre17/boxpiletas.html), [Fn](http://doc.rust-lang.org/std/ops/trait.Fn.html), [FnMut](http://doc.rust-lang.org/std/ops/trait.FnMut.html) et [la généricité](../chapitre12/genericite.html).