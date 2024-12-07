macro_rules! magic_spelling {
    (fire) => {
        println!("FIRE!")
    };

    (water) => {
        println!("WATER!")
    };
}

fn main() {
    magic_spelling!(water);
    magic_spelling!(fire);
}
