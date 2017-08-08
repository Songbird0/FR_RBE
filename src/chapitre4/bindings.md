# Les assignations

Rust assure l'immuabilité du type d'une variable grâce au typage statique. Lorsqu'une variable est déclarée elle peut être typée. Cependant, dans la plupart des cas, le compilateur sera capable d'inférer le type de la variable en se basant sur le contexte, atténuant sérieusement la lourdeur du typage.

Les valeurs (tels que les littéraux) peuvent être assignées à des variables en utilisant le mot-clé `let`.

{{#playpen source/bindingssource0.rs}}
