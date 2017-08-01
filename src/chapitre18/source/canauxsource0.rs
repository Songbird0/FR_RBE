use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // Les canaux possèdent deux extrémités: l'envoyeur (`Sender<T>`) et le receveur (`Receiver<T>`),
    // où `T` est le type du message à envoyer.
    // (Le typage est optionnel)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // L'envoyeur peut être copié.
        let thread_tx = tx.clone();

        // Chaque thread va envoyer son identifiant par le biais du canal.
        thread::spawn(move || {
            // Le thread prend l'ownership sur `thread_tx`.
            // Chaque thread va ajouter un message dans le file d'attente 
            // dans le canal.
            thread_tx.send(id).unwrap();

            // L'envoi est une opération non-bloquante, le thread continuera à 
            // s'exécuter après l'envoi de son message.
            println!("thread {} finished", id);
        });
    }

    // Ici, on récupère tous les messages.
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // La méthode `recv` choisit une message se trouvant dans le canal et 
        // gêlera le thread courant s'il n'y a aucun message.
        ids.push(rx.recv());
    }

    // Montre l'ordre dans lequel les messages ont été envoyés.
    println!("{:?}", ids);
}
