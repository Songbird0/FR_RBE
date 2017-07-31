# La lifetime `'static`

Une ressource annotée du label `‘static` possède la plus longue durée de vie qu'on puisse assigner. La durée de vie de `‘static` est égale à celle du programme lui-même et peut également être raccourcie selon le contexte.

Il y a deux façons d'assigner la lifetime `‘static`, et toutes deux stockeront la ressource directement dans le binaire en lecture seule :

1. Créer une constante avec la déclaration `static`;
2. Créer une chaîne de caractères avec le `"littéral"` en l'annotant du type `&'static str`.

Voici un exemple pour illustrer les deux manières de faire :

{{#playpen source/staticsource0.rs}}

## Voir aussi

[Les constantes](../chapitre3/constantes.html).
