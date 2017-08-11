# Le trait Clone

Lors du traitement des ressources, le comportement par défaut est de les transférer lors d'un assignement ou un appel de méthode. Cependant, il est parfois nécessaire d'effectuer une copie des ressources en question.

C'est exactement la fonction du trait [`Clone`][clone_trait], qui nous permettra d'utiliser la méthode `clone()`.

{{#playpen source/traitclonesource0.rs}}

[clone_trait]: https://doc.rust-lang.org/std/clone/trait.Clone.html
