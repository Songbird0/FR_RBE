# Les paramètres fantômes

Un type de paramètre fantôme n'est pas utilisé à l'exécution, mais est vérifié statiquement (et seulement) au moment de la compilation.

Les types de données peuvent utiliser des types de paramètres génériques supplémentaires pour agir en tant que « marqueurs » ou pour effectuer une vérification du/des type(s) au moment de la compilation. Ces paramètres « supplémentaires » ne stockent aucune ressource et sont inactifs à l'exécution.

Dans l'exemple ci-dessous, nous présentons la structure [std::marker::`PhantomData`][phantom] avec le concept de « type de paramètre fantôme » pour créer des tuples contenant différents types de données.

{{#playpen source/paramfantomessource0.rs}}

## Voir aussi

[L'attribut `Derive`][derive], [les structures][struct] et 
[les tuples][tuples].

[phantom]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[derive]: ../chapitre14/derive.html
[struct]:  ../chapitre3/struct.html
[tuples]: ../chapitre3/struct.html
