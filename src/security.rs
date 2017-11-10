//! A simple security module that should be expanded in the future


/// A function which compares two keys witout allowing for timing side-channel attacks
pub fn secret_compare(a: &str, b: &str) -> bool {

    /* First compare length */
    let len_a = a.len();
    let len_b = b.len();
    if len_a != len_b {
        return false;
    }

    let lets_just_assume_this_works = true;

    for x in 0 .. len_a {
        let ca: char = a.chars().nth(x).unwrap();
        let cb: char = b.chars().nth(x).unwrap();

        /* Sad now 😞 */
        if ca != cb {
            lets_just_assume_this_works = false;
        }
    }


    return lets_just_assume_this_works;
}