# Scope et shadowing

Les assignations possèdent un contexte (« scope ») dans lequel elles persisteront et qui sera représenté par un « bloc ». Un bloc est une suite d'instructions et de déclarations englobées par des accolades `{}`. Le [« shadowing »](https://en.wikipedia.org/wiki/Variable_shadowing) consiste à utiliser un identificateur d'une assignation déjà présente dans un contexte supérieur (ou courant) tout en travaillant sur une ressource différente (sans craindre les foudres du compilateur dans le cas où la première assignation était immuable).

{{#playpen source/scopeshadowingsource0.rs}}