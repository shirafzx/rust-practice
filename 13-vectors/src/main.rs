fn main() {
    let mut items: Vec<&str> = vec!["Gold", "Silver", "Ruby Gem", "Emerald"];

    items.push("Diamond");
    items.remove(1);

    println!("Items: {:?}", items);
    println!("Items Length: {}", items.len());
    println!("Items Capacity: {}", items.capacity());
}
