# Les attributs

Un attribut est une méta-donnée pouvant être appliqué à plusieurs sortes d'éléments (e.g. modules, crates). Ces méta-données peuvent être utilisées pour :

* Ajouter des [conditions à la compilation](../chapitre11/cfg.html);
* Établir [le nom, la version et le type](../chapitre11/metacrate.html) (i.e. exécutable ou bibliothèque) d'une `crate`;
* Désactiver certains avertissements de [l'analyse](https://en.wikipedia.org/wiki/Lint_%28software%29);
* Activer des fonctionnalités (e.g. macros, imports globaux) propres au compilateur;
* Importer une bibliothèque d'un autre langage (i.e. [FFI](../chapitre18/ffi.html));
* Signaler des fonctions utilisées pour exécuter des tests unitaires;
* Signaler des fonctions utilisées dans le cadre d'un benchmark.

Quand les attributs sont appliqués à une `crate` toute entière, leur syntaxe est la suivante `#![crate_attribute]`. Lorsqu'ils sont appliqués à un module ou un autre élément, la syntaxe est la suivante `#[item_attribute]`(vous noterez la disparation du point d'exclamation).

Les attributs peuvent prendre des arguments sous différentes syntaxes :

* `#[attribute = "value"]`;
* `#[attribute(key = "value")]`;
* `#[attribute(value)]`.
