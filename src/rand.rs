use std::iter;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn gen_random_alphanum(len: usize) -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect();
    return chars;
}
