fn main() {
    let treasures = ["Gold", "Silver", "Crystal Gem", "Emerald", "Iron", "Bronze", "Diamond"];
    let mut energy: i32 = 5;

    for treasure in treasures.iter() {
        if energy == 0 {
            println!("You are out of energy. Game Over!");
            break;
        } else if treasure == &"Ruby Gem" {
            println!("You found the Ruby Gem. You win!");
            break;
        }

        energy -= 1;
    }
}
