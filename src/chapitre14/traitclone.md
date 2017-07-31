# Le trait Clone

Lors du traitement des ressources, le comportement par défaut est de les transférer lors d'un assignement ou un appel de méthode. Cependant, il est parfois nécessaire d'effectuer une copie des ressources en question.

C'est exactement la fonction du trait [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html). Plus communément, nous pouvons utiliser la méthode `.clone()` implémentée par le trait `Clone`.

{{#playpen source/traitclonesource0.rs}}
