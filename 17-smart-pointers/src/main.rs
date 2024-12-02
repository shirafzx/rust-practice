use std::{cell::RefCell, rc::Rc};

fn main() {
    // let chest: Box<i32> = Box::new(10);
    // let shared_chest: Rc<RefCell<Box<i32>>> = Rc::new(RefCell::new(chest));

    // **shared_chest.borrow_mut() += 10;
    // **shared_chest.borrow_mut() += 5;

    let chest: i32 = 10;
    let shared_chest: Rc<RefCell<i32>> = Rc::new(RefCell::new(chest));
    
    *shared_chest.borrow_mut() += 10;
    *shared_chest.borrow_mut() += 5;

    println!("Gold: {}", shared_chest.borrow());
}
