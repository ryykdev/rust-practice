use std::fmt::Debug;

// --- 1. Function constrained to Sized types (The Default) ---

// The type parameter T is implicitly constrained to T: Sized.
// T is passed by value (move).
fn process_sized_debug<T: Debug>(item: T) {
    println!("Sized Function (T: Sized): Received item: {:?}", item);
}

// --- 2. Function allowing ?Sized types ---

// By using T: ?Sized, we remove the Sized constraint, allowing 'T' to be a DST (like 'str').
// Because the type might be unsized, the item MUST be passed by reference (&T).
fn process_unsized_debug<T: Debug + ?Sized>(item: &T) {
    println!(
        "?Sized Function (T: ?Sized): Received item (via reference): {:?}",
        item
    );
}

fn main() {
    // ------------------------------------------
    // A. Using Sized Type (String)
    // ------------------------------------------
    let owned_string = "hello sized world".to_string(); // Sized: Yes (String)

    // ✅ SUCCESS: T is String, which is Sized. 'owned_string' is moved.
    process_sized_debug(owned_string);

    let string_for_borrow = "another sized string".to_string();
    // ✅ SUCCESS: T is String (Sized). It is passed as a reference (&String).
    process_unsized_debug(&string_for_borrow);

    // ------------------------------------------
    // B. Using Dynamically Sized Reference (The point of confusion)
    // ------------------------------------------
    let string_slice: &str = "hello unsized slice";

    // ✅ SUCCESS: T is inferred as &str. The reference &str IS a Sized type
    // (a 16-byte fat pointer), so it can be moved by value into the Sized function.
    process_sized_debug(string_slice);

    // ✅ SUCCESS: T is inferred as str. The function is happy because 'str' satisfies
    // the ?Sized bound, and we passed a pointer (&str) which is required by the function signature.
    process_unsized_debug(string_slice);

    // ------------------------------------------
    // C. The True Failure Case (Passing the DST itself)
    // ------------------------------------------

    // The only time the Sized function strictly fails is if we try to move the
    // raw Dynamically Sized Type (str or [u8]) by value, which is impossible
    // without unsafe methods.

    // let number_slice: [u8] = [10, 20, 30]; // ERROR: Cannot create a value of DST type [u8]

    // Example using Box<T>: Box is Sized, but its content is often used with DSTs.
    let boxed_slice: Box<[u8]> = Box::new([40, 50, 60]);

    // This is the ideal demonstration of the Sized vs ?Sized difference:
    // T is Box<[u8]>. Box is Sized, so it works.
    process_sized_debug(boxed_slice);
    process_sized_debug([10, 20, 30]);

    // If you uncomment the line below, it would fail because the function expects &T,
    // but the actual DST type is [u8], and &Box<[u8]> is not &str.
    // process_unsized_debug(&boxed_slice);

    // ✅ SUCCESS: The underlying DST ([u8]) can be passed to the ?Sized function
    // when referenced, as it uses the fat pointer.
    process_unsized_debug(&[10, 20, 30]);

    println!("{:?}", [10, 20, 30]);
}
