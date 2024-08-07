use iter_ref::*;

struct YourFridge {
    food: Vec<String>
}

fn expire(foods: &mut impl IterRefMut<Item = String>) {
    for food in foods.iter_better_mut() {
        *food = ["Expired ", food].concat()
    }
}

fn main() {
    let mut fridge = YourFridge { food: vec![
        "banana".to_string(),
        "eggs".to_string(),
        "peanutbutter".to_string(),
    ] };

    expire(&mut fridge.food);

    for food in fridge.food {
        println!("{}", food)
    }
}