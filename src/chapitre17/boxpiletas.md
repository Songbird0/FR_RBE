# Les `Box`, la pile et le tas

En Rust, toutes les valeurs sont allouées dans la pile, par défaut. Les valeurs peuvent être *boxées* (i.e. allouées dans le tas) en créant une instance de `Box<T>`. Une "box" est un pointeur intelligent sur une ressource de type `T` allouée dans le tas. Lorsqu'une box sort du contexte, son destructeur est appelé, l'objet à charge est détruit et la mémoire du tas libérée.

Les valeurs "boxées" peuvent être déréférencées en utilisant l'opérateur `*`; Ceci supprime un [niveau d'indirection][indirection].

{{#playpen source/boxpiletassource0.rs}}

[indirection]: https://stackoverflow.com/questions/18003544/what-does-level-of-indirection-mean-in-david-wheelers-aphorism/18003704#18003704
