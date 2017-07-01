fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // Déstructure les membres de la structure.
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // Vous pouvez déstructurer les structures et renommer 
    // leurs variables. L'ordre n'est pas important.

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // et vous pouvez aussi ignorer certaines variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // Ceci donne une erreur: le pattern ne mentionne pas le champ `x`.
    // let Foo { y } = foo;
}