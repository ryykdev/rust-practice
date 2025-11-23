fn main() {
    let mut fruits = vec![
        "strawberry".to_string(),
        "mango".to_string(),
        "mandarin".to_string(),
        "watermelon".to_string(),
    ];
    for fruit in fruits.iter_mut() {
        if let Some(first_letter) = fruit.chars().next() {
            let first = first_letter.to_uppercase();

            let rest_of_word = &fruit[1..];

            *fruit = first.to_string() + rest_of_word;
        }
    }
    let nuts = vec![
        "walnut".to_string(),
        "pistacio".to_string(),
        "macadamia".to_string(),
        "peanut".to_string(),
    ];

    let foods = fruits.iter().chain(&nuts);
    for food in foods.clone() {
        println!("chained: {food}");
    }
    // stepping over 2
    for food in foods.step_by(2) {
        println!("with step: {food}");
    }

    // iter mut
    for fruit in fruits.iter_mut() {
        *fruit = format!("fruits: {fruit}");
    }
    dbg!(&fruits);

    // iter mut
    let nuts_prefixed = nuts.iter().map(|nut| format!("nut: {nut}"));
    dbg!(&nuts_prefixed);

    let zipped = fruits.iter().zip(&nuts);
    zipped.for_each(|item: (_, _)| println!("first: {}, second: {}", item.0, item.1));
}
