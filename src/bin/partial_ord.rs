fn main() {
    let k = Rank::King;
    let p = Rank::Prince;

    if k > p {
        println!("King outranks Prince");
    }
}

#[derive(PartialEq)]
enum Rank {
    King,
    Queen,
    Prince,
    Princess,
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let get_rank_value = |rank: &Rank| match rank {
            Rank::King => 3,
            Rank::Queen => 2,
            Rank::Prince => 1,
            Rank::Princess => 0,
        };

        let self_value = get_rank_value(self);
        let other_value = get_rank_value(other);

        self_value.partial_cmp(&other_value)
    }
}
