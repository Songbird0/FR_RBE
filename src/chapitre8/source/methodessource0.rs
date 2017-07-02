struct Point {
    x: f64,
    y: f64,
}

// Toutes les méthodes de la structure `Point` sont implémentées ici.
impl Point {
    // Ceci est une méthode statique.
    // Les méthodes statiques ne sont pas dépendantes des instances.
    // Il est courant d'utiliser les méthodes statiques comme constructeurs.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Une autre méthode statique possédant
    // deux paramètres.
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Ceci est une méthode dépendante d'une instance (méthode d'instance).
    // `&self` est le sucre syntaxique de `self: &Self` où `Self` est le 
    // type de l'objet qui appelle les méthodes. En l'occurrence 
    // `Self` = `Rectangle`.
    fn area(&self) -> f64 {
        // `self` donne accès aux champs de la structure via la notation pointée.
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` est une méthode renvoyant un réel de type f64 qui représente 
        // la valeur absolue du primitif.
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Cette méthode a besoin d'opérer sur une référence mutable 
    // de l'instance courante `&mut self`.
    // La syntaxe non-raccourcie est `self: &mut Self`.
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` possède deux entiers alloués dans le tas.
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Cette méthode "consomme" les ressources de l'instance courante.
    // `self` est un sucre syntaxique de `self: Self`.
    fn destroy(self) {
        // Déstructure `self` (i.e. récupère les champs désirés).
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // La mémoire occupée par `first` et `second` sera libérée
        // une fois que l'exécution de la méthode prendra fin.
    }
}

fn main() {
    let rectangle = Rectangle {
        // Les méthodes statiques sont appelées par le biais 
        // d'une paire de deux points `::`.
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Vous devez, en revanche, utiliser la notation pointée pour appeler 
    // les méthodes d'instance. 
    // Notez que le premier argument passé (implicitement) est `&self`
    // (i.e. `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`).
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Erreur! `rectangle` est immuable alors que cette méthode 
    // nécessite une référence mutable de l'objet.
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Essayez de décommenter cette ligne.

    // C'est bon! Les objets mutables peuvent appeler les méthodes
    // "mutables".
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Erreur! L'appel de la méthode `destroy` a consummé
    // l'instance `pair`.
    //pair.destroy();
    // TODO ^ Essayez de décommenter cette ligne.
}
