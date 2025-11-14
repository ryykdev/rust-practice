use std::ops::Add;
fn main() {
    let bill = GroceryBill::default();
    let butter = GroceryItem {
        name: "butter".to_string(),
        price_cents: 270,
    };
    let toast = GroceryItem {
        name: "toast".to_string(),
        price_cents: 199,
    };
    let new = bill + butter + toast;
    dbg!(new);
}

#[derive(Default, Debug)]
pub struct GroceryBill {
    items: Vec<GroceryItem>,
    total: u32,
}

#[derive(Default, Debug)]
pub struct GroceryItem {
    name: String,
    price_cents: u32,
}

impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;

    fn add(mut self, item: GroceryItem) -> Self::Output {
        self.total += item.price_cents;
        self.items.push(item);

        self
    }
}

impl Add<GroceryBill> for GroceryBill {
    type Output = GroceryBill;

    fn add(self, rhs: GroceryBill) -> Self::Output {
        self.total +=
    }
}
