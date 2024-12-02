trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

// fn use_gear<T: Gear>(item: T) {
//     item.use_gear();
// }

// fn use_gear(item: Box<dyn Gear>) {
//     item.use_gear();
// }

fn use_gear(item: &impl Gear) {
    item.use_gear();
}

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Fire arrow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drink potion");
    }
}

fn main() {
    let crabby_sword: Sword = Sword;
    let crabby_bow: Bow = Bow;
    let crabby_potion: Potion = Potion;

    // let crabby_sword: Box<Sword> = Box::new(Sword);
    // let crabby_bow: Box<Bow> = Box::new(Bow);
    // let crabby_potion: Box<Potion> = Box::new(Potion);

    use_gear(&crabby_sword);
    use_gear(&crabby_bow);
    use_gear(&crabby_potion);
}
