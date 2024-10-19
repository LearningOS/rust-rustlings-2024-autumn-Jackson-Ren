// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */(name, age) = cat;

    println!("{} is {} years old.", name, age);

    let person = ("Jacky", "JiangSu", 18, true);
    let /* your pattern here */(name, local, age, is_man) = person;

    println!("{} is {} years old, come from {}, {}", name, age, local, if is_man {"is a man"} else {"is a woman"});
}
