# Déclaration seule

Il est possible de déclarer une variable dans un premier temps, pour l'initialiser dans un second temps. Cependant, cette forme est rarement utilisée puisqu'elle peut conduire à l'utilisation de variables qui ne sont pas initialisées (et donc à faire des erreurs).

{{#playpen source/declarationseulesource0.rs}}

Comme l'utilisation d'une variable, qui n'a pas été initialisée au préalable, peut mener à des comportements imprévisibles à l'exécution, le compilateur vous interdit de les utiliser.
