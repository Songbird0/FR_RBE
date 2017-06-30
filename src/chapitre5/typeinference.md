# L'inférence des types

Le moteur dédié à l'inférence des types est très intelligent. Il fait bien plus que d'inférer le type d'une `r-value` à l'initialisation. Il se charge également d'analyser l'utilisation de la variable dans la suite du programme pour inférer son type définitif. Voici un exemple plus avancé dédié à l'inférence :

{{#playpen source/typeinferencesource0.rs}}

Aucun typage explicite n'était nécessaire, le compilateur est heureux et le programmeur aussi !