# Répétition

Les macros peuvent utiliser le quantificateur `+` dans la liste des arguments pour indiquer qu'un argument peut être répété au moins une(1) fois ou `*` pour indiquer que l'argument peut être répété zéro(0) ou plusieurs fois.

Dans l'exemple suivant, entourer le matcher avec `$(...),+` va permettre la capture d'une ou plusieurs expressions, séparées par des virgules. Notez également que le point-virgule est optionnel dans le dernier cas (i.e. la dernière expression capturée).

{{#playpen source/repetitionregexpsource0.rs}}
