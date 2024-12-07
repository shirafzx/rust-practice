use modules_and_crates::{potions, weapons::use_item};

mod maps {
    pub fn use_item() {
        println!("You used a map!");
    }
  }

fn main() {
    potions::use_item();
    use_item::use_item();
    maps::use_item();
}
