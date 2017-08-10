# Système d'emprunts

Très souvent, nous souhaiterions accéder à une ressource sans en prendre possession. Pour ce faire, Rust utilise un *système d'emprunts*. Plutôt que de passer un objet *par valeur* (`T`), il peut être passé par référence (`&T`).

Le compilateur garantit (grâce au vérificateur d'emprunts) que les références sont toujours valides. Tant qu'une référence de l'objet existe, il ne sera pas détruit.

{{#playpen source/borrowingsource0.rs}}
