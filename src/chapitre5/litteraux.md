# Les littéraux

Les littéraux numériques peuvent être typés en suffixant le littéral avec son type. Par exemple, pour préciser que le littéral `42` devrait posséder le type `i32`, nous écrirons `42i32`.

Le type des littéraux numériques qui ne sont pas suffixés va dépendre du contexte dans lequel ils sont utilisés. Si il n'y a aucune contrainte (i.e. si rien ne force la valeur à être codée sur un nombre de bits bien précis), le compilateur utilisera le type `i32` pour les entiers et `f64` pour les nombres réels.

{{#playpen source/litterauxsource0.rs}}

Certains concepts présentés dans l'exemple ci-dessus n'ont pas encore été abordés. Pour les plus impatients, voici une courte explication :


*  `fun(&foo)` : Cette syntaxe représente le passage d'un paramètre par référence plutôt que par valeur (i.e. `fun(foo)`). Pour plus d'informations, voir [le chapitre du système d'emprunts](../chapitre13/borrowing.html).

 
* `std::mem::size_of_val` est une fonction mais appelée avec son chemin absolu. Le code peut être divisé et organisé en plusieurs briques logiques nommées *modules*. Pour le cas de la fonction `size_of_val`, elle se trouve dans le module `mem`, lui-même se trouvant dans le paquet `std`. Pour plus d'informations voir [les modules](../chapitre9/module.html) et/ou [les « crates »](../chapitre10/crate.html).