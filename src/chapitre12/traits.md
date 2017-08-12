# Les traits

Bien entendu, les traits peuvent également être génériques. Dans cette section, nous allons en créer un qui ré-implémente le trait `Drop` qui proposera une méthode générique qui aura pour fonction de libérer l'instance qui l'appelle ainsi qu'un paramètre passé.

{{#playpen source/traitssource0.rs}}

## Voir aussi

[La documentation du trait Drop][drop], [le chapitre sur les structures][struct] et [les traits][traits].

[drop]: http://doc.rust-lang.org/std/ops/trait.Drop.html
[struct]: ../chapitre3/struct.html
[traits]: ../chapitre14/traits.html
