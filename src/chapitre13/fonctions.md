# Les fonctions

Lorsque le concept [d'élision](../chapitre13/elision.html) ne peut pas être appliqué, les signatures de fonctions comportant des (labels de) lifetimes sont régies par quelques règles :

1. Toute référence doit être annotée d'une lifetime;
2. Toute référence renvoyée doit posséder la même lifetime qu'en entrée ou la lifetime `‘static`.

Notez également que, si une fonction n'a pas de références en entrée, le renvoi de références est interdit car cela pourrait conduire à l'utilisation de ressources invalides.

{{#playpen source/fonctionssource0.rs}}

## Voir aussi

[Les fonctions](../chapitre8/fonctions.html).
