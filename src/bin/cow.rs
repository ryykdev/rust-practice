use std::borrow::Cow;

// This function can accept borrowed data (cheap) or owned data.
fn ensure_uppercase(mut data: Cow<str>) -> Cow<str> {
    // Check the first character. If it's lowercase, we need to modify it.
    if data.starts_with(|c: char| c.is_ascii_lowercase()) {
        // 🚨 This is the "Clone-on-Write" moment! 🚨
        // data.to_mut() converts the Cow<str> into a mutable owned String (&mut String).
        // 1. If 'data' was Borrowed, it is CLONED into a new String on the heap.
        // 2. If 'data' was already Owned, the owned data is simply returned mutably.
        let owned_string = data.to_mut();

        // Perform the modification on the now-owned data
        if let Some(c) = owned_string.chars().next() {
            let upper = c.to_ascii_uppercase().to_string();
            owned_string.replace_range(..c.len_utf8(), &upper);
        }

        // The Cow is now inherently owned (because we mutated it) and is returned.
        data
    } else {
        // If no mutation is needed, return the original data as-is (Borrowed or Owned)
        data
    }
}

fn main() {
    // Case A: Borrowed Data (The cheap path)
    let b: Cow<str> = Cow::Borrowed("hello");
    let result_b = ensure_uppercase(b);

    // The result is Cow::Owned because a clone had to happen to perform the mutation.
    println!("Case A (Borrowed -> Owned): {}", result_b);

    // Case B: Already Owned Data (No extra clone needed)
    let o: Cow<str> = Cow::Owned("world".to_string());
    let result_o = ensure_uppercase(o);

    // The result is Cow::Owned, but no *extra* clone occurred inside ensure_uppercase.
    println!("Case B (Owned -> Owned): {}", result_o);

    // Case C: Already Correctly Formatted (The cheapest path)
    let c: Cow<str> = Cow::Borrowed("Hello");
    let result_c = ensure_uppercase(c);

    // The result is Cow::Borrowed because no mutation was needed.
    println!("Case C (Borrowed -> Borrowed): {}", result_c);
}
