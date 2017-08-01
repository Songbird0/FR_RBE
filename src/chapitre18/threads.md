# Les fils d'exécution

Rust fournit un méchanisme de création de fils d'exécution natifs via la fonction `spawn`. L'argument de cette fonction est une closure transférable.

{{#playpen source/threadssource0.rs}}

Ces threads seront programmés par le système d'exploitation.
