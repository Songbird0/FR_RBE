# La visibilité des structures

Les structures disposent d'un niveau supplémentaire de visibilité dédié à leurs champs. Comme pour les autres ressources, les champs d'une structure sont privés par défaut, mais peuvent être rendus publics en utilisant, encore une fois, le mot-clé `pub`. La visibilité des champs ne s'applique, bien entendu, que lorsqu'une structure est sollicitée en dehors du module où elle a été déclarée et a pour but de masquer les données ([encapsulation][encap]).

{{#playpen source/visibilitestructsource0.rs}}

## Voir aussi

[La généricité][genericite] et [les méthodes][methodes].

[encap]: https://fr.wikipedia.org/wiki/Encapsulation_(programmation)
[genericite]: ../chapitre12/genericite.html
[methodes]: ../chapitre8/methodes.html
