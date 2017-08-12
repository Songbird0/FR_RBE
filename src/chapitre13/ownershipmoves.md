# Ownership et transferts

Parce que les variables sont responsables de la libération de leurs ressources, **les ressources ne peuvent avoir qu'un seul propriétaire/responsable**.

Cette règle évite également au développeur de libérer plus d'une fois une ressource. Notez toutefois que toutes les variables ne possèdent pas leurs propres ressources (e.g. [les références][ref]).

Lorsque nous assignons quelque chose à une variable (`let x = y`) ou passons un (ou des) argument à une fonction par valeur (`foo(x)`), l'ownership des ressources est transféré. Dans le jargon, cette action est nommée « `move` » (transfert).

Après avoir transféré une ressource, l'ancien propriétaire ne peut plus être utilisé. Cela prévient la création de « dangling pointers » (i.e. des pointeurs sur une ressource qui n'est plus valide).

{{#playpen source/ownershipmovessource0.rs}}

[ref]: ../chapitre7/pointeuref.html
