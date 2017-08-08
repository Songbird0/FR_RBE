#![allow(unreachable_code)] // permet de faire taire les avertissements 
// relatifs au code mort.

fn main() {
    'externe: loop {
        println!("Entré dans la boucle annotée 'externe.");

        'interne: loop {
            println!("Entré dans la boucle annotée 'interne.");

            // Cette instruction nous ferait simplement 
            // sortir de la boucle 'interne.
            // break;
            
            // On sort de la boucle 'externe 
            // à partir de la boucle 'interne.
            break 'externe;
        }

        println!("Cette ligne ne sera jamais exécutée.");
    }

    println!("Sorti de la boucle annotée 'externe.");
}
