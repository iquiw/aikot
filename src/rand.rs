use std::iter;

use rand::Rng;
use rand::distr::Alphanumeric;
use rand::rngs::ThreadRng;

pub fn gen_random_alphanum(len: usize) -> String {
    let mut rng = ThreadRng::default();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect();
    chars
}
