fn main() {
    let coins;

    {
        let spending = 5u32;
        coins = &spending;
    }

    println!("r: {}", coins);

    // This results in the error:

    /*
        error[E0597]: `spending` does not live long enough
        --> October-13-2023/invalid_pointer/src/main.rs:6:17
        |
      5 |         let spending = 5u32;
        |             -------- binding `spending` declared here
      6 |         coins = &spending;
        |                 ^^^^^^^^^ borrowed value does not live long enough
      7 |     }
        |     - `spending` dropped here while still borrowed
      8 |
      9 |     println!("r: {}", coins);
        |                       ----- borrow later used here

        For more information about this error, try `rustc --explain E0597`.
        error: could not compile `invalid_pointer` (bin "invalid_pointer") due to previous error
    */
}
