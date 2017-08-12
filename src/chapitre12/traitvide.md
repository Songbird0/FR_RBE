# Exemple d'utilisation : Traits sans services

Puisqu'il est possible d'imposer des conditions aux types génériques grâce aux traits, même si ces derniers ne possèdent aucune fonctionnalité (i.e. aucun service, aucune méthode), il est toujours possible de vous en servir comme simple « filtre ». `Eq` et `Ord` font partie de ces traits « vides » fournis par la bibliothèque standard.

{{#playpen source/traitvidesource0.rs}}

## Voir aussi

La documentation du trait [Eq][eq], la documentation du trait [Ord][ord] et [les traits][traits].

[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[traits]: ../chapitre14/traits.html
