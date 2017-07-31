# La macro `panic`

Le plus simple mécanisme de gestion d'erreur que nous allons voir est `panic`. Il affiche un message d'erreur, lance la tâche et généralement met fin au programme. Ici, nous appelons explicitement `panic` dans notre condition:

{{#playpen source/panicsource0.rs}}
