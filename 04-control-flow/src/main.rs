fn main() {
    let weather: &str = "rainy";
    if weather == "sunday" {
        println!("Crabby will cross the river by swimming!")
    } else if weather == "rainy" {
        println!("Crabby will build a bridge to stay dry.")
    } else if weather == "stormy" {
        println!("Crabby will wait for better weather.")
    }

    let enemy: &str = "troll";
    match enemy {
        "goblin" => println!("Crabby uses his rusty sword to attack."),
        "troll" => println!("Crabby sets a trap!"),
        "dragon" => println!("Crabby runs for a cover!"),
        _ => println!("Crabby is confused..."),
    }

    let mut wood: i32 = 0;
    loop {
        wood += 1;
        // println!("Crabby gathered {} pirces of wood.", wood);
        let current_wood: &str = "Crabby gathered {wood} pirces of wood.";
        println!("{current_wood}");
        // println!("Crabby gathered {wood} pirces of wood.");
        if wood == 10 {
            println!("Crabby finished the boat!");
            break;
        }
    }
}
