fn main() {
    let k = Rank::King;
    let k2 = Rank::King;
    let q = Rank::Queen;

    println!("k == k2 is {}", k == k2);
    println!("k == q is {}", k == q);
}

enum Rank {
    King,
    Queen,
    Prince,
    Princess,
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        //match (self, other) {
        //    (Rank::King, Rank::King) => true,
        //    (Rank::Queen, Rank::Queen) => true,
        //    (Rank::Prince, Rank::Prince) => true,
        //    (Rank::Princess, Rank::Princess) => true,
        //    _ => false,
        //}
        matches!(
            (self, other),
            (Rank::King, Rank::King)
                | (Rank::Queen, Rank::Queen)
                | (Rank::Prince, Rank::Prince)
                | (Rank::Princess, Rank::Princess)
        )
    }
}
