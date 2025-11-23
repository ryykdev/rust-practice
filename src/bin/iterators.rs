fn main() {
    let mut fruits = vec![
        "strawberry".to_string(),
        "mango".to_string(),
        "mandarin".to_string(),
        "watermelon".to_string(),
    ];
    let nuts = vec![
        "walnut".to_string(),
        "pistacio".to_string(),
        "macadamia".to_string(),
        "peanut".to_string(),
    ];

    let foods = fruits.iter().chain(&nuts);

    for food in foods {
        println!("{food}");
    }

    // iter mut
    for fruit in fruits.iter_mut() {
        *fruit = format!("fruits: {fruit}");
    }

    dbg!(fruits);
}
