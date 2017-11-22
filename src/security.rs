//! A simple security module that should be expanded in the future


#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct Signature {
    pub signature: String,
    pub key_id: String,
}


/// A function which compares two keys witout allowing for timing side-channel attacks
///
/// Provide it with two strings that are the same length
pub fn secret_compare(a: &str, b: &str) -> bool {

    /* First compare length */
    let len_a: i32 = a.len();
    let len_b: i32 = b.len();
    if len_a != len_b {
        println!("Not the same length {} {}", len_a, len_b);
        return false;
    }

    let mut lets_just_assume_this_works = true;
    for x in 0..len_a {
        let ca: char = a.chars().nth(x).unwrap();
        let cb: char = b.chars().nth(x).unwrap();

        /* Sad now 😞 */
        if ca != cb {
            lets_just_assume_this_works = false;
        }
    }

    return lets_just_assume_this_works;
}


/// A lovely small function which will generate a key secret for you
#[allow(unused)]
pub fn secret_generate() -> String {
    unimplemented!();
}
