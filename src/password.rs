use std::fmt;
use std::iter;

use rand;
use rand::distributions::{Alphanumeric, Distribution};
use rand::{thread_rng, Rng};

use crate::err::AikotError;

const SYMBOL_CHARS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub struct Alphanum;
pub struct AlphanumSymbol;

pub enum PwGen {
    AN(Alphanum, usize),
    ANS(AlphanumSymbol, usize),
}

impl PwGen {
    pub fn new(length: usize, symbol: bool) -> Result<Self, AikotError> {
        let pwgen = if symbol {
            PwGen::ANS(AlphanumSymbol, length)
        } else {
            PwGen::AN(Alphanum, length)
        };

        let min_len = pwgen.minimum_length();
        if length < min_len {
            Err(AikotError::MinimumLength {
                pwgen: format!("{}", pwgen).to_string(),
                min_len,
            })?
        } else {
            Ok(pwgen)
        }
    }

    pub fn try_generate(&self) -> Result<String, AikotError> {
        let opass = match self {
            PwGen::AN(x, len) => x.try_generate(*len),
            PwGen::ANS(x, len) => x.try_generate(*len),
        };

        if let Some(pass) = opass {
            Ok(pass)
        } else {
            Err(AikotError::GenerationFail {
                pwgen: format!("{}", self).to_string(),
            })?
        }
    }

    fn minimum_length(&self) -> usize {
        match self {
            PwGen::AN(x, _) => x.minimum_length(),
            PwGen::ANS(x, _) => x.minimum_length(),
        }
    }
}

impl fmt::Display for PwGen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PwGen::AN(_, len) => write!(f, "length: {}, class: alphanum", len),
            PwGen::ANS(_, len) => write!(f, "length: {}, class: alphanum+symbol", len),
        }
    }
}

pub trait PasswordClass: Distribution<char> {
    fn try_generate(&self, len: usize) -> Option<String>
    where
        Self: Sized,
    {
        let mut rng = thread_rng();
        for _i in 1..100 {
            let pass: String = iter::repeat(())
                .map(|()| rng.sample(self))
                .take(len)
                .collect();
            if self.verify(&pass) {
                return Some(pass);
            }
        }
        None
    }

    fn minimum_length(&self) -> usize;

    fn verify(&self, pass: &str) -> bool;
}

impl Distribution<char> for Alphanum {
    fn sample<R>(&self, rng: &mut R) -> char
    where
        R: rand::Rng + ?Sized,
    {
        Alphanumeric.sample(rng)
    }
}

impl PasswordClass for Alphanum {
    fn minimum_length(&self) -> usize {
        3
    }

    fn verify(&self, pass: &str) -> bool {
        let preds: Vec<Box<dyn Fn(char) -> bool>> = vec![
            Box::new(lower_char),
            Box::new(upper_char),
            Box::new(number_char),
        ];
        all_predicts(&preds, pass)
    }
}

impl Distribution<char> for AlphanumSymbol {
    fn sample<R>(&self, rng: &mut R) -> char
    where
        R: rand::Rng + ?Sized,
    {
        // Derived from Alphanum code in rand/distributions/other.rs
        const RANGE: u32 = 26 + 26 + 10 + 32;
        const GEN_ALPHANUM_SYMBOL_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789\
                !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        loop {
            let var = rng.next_u32() >> (32 - 7);
            if var < RANGE {
                // succeeds, 94 / 128 = 73%
                return GEN_ALPHANUM_SYMBOL_CHARSET[var as usize] as char;
            }
        }
    }
}

impl PasswordClass for AlphanumSymbol {
    fn minimum_length(&self) -> usize {
        4
    }

    fn verify(&self, pass: &str) -> bool {
        let preds: Vec<Box<dyn Fn(char) -> bool>> = vec![
            Box::new(lower_char),
            Box::new(upper_char),
            Box::new(number_char),
            Box::new(symbol_char),
        ];
        all_predicts(&preds, pass)
    }
}

fn lower_char(c: char) -> bool {
    c.is_lowercase()
}

fn upper_char(c: char) -> bool {
    c.is_uppercase()
}

fn number_char(c: char) -> bool {
    c.is_numeric()
}

fn symbol_char(c: char) -> bool {
    SYMBOL_CHARS.contains(c)
}

fn all_predicts<F>(preds: &Vec<F>, s: &str) -> bool
where
    F: Fn(char) -> bool,
{
    let mut passed = vec![false; preds.len()];
    for c in s.chars() {
        for i in 0..preds.len() {
            if preds[i](c) {
                passed[i] = true;
            }
        }
        if passed.iter().all(|b| *b) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_predicts_for_empty_pred() {
        let mut v: Vec<Box<dyn Fn(char) -> bool>> = vec![];
        assert!(all_predicts(&mut v, "foo"));
    }

    #[test]
    fn test_all_predicts_for_one_pred_pass() {
        let mut v = vec![|c| c == 'f'];
        assert!(all_predicts(&mut v, "foo"));
    }

    #[test]
    fn test_all_predicts_for_one_pred_fail() {
        let mut v = vec![|c| c == 'g'];
        assert!(!all_predicts(&mut v, "foo"));
    }

    #[test]
    fn test_all_predicts_for_one_pred_pass_of_3preds() {
        let mut v: Vec<Box<dyn Fn(char) -> bool>> = vec![
            Box::new(|c| c == 'g'),
            Box::new(|c| c == 'h'),
            Box::new(|c| c == 'f'),
        ];
        assert!(!all_predicts(&mut v, "foo"));
    }

    #[test]
    fn test_all_predicts_for_all_preds_pass_of_3preds() {
        let mut v: Vec<Box<dyn Fn(char) -> bool>> = vec![
            Box::new(|c| c == 'f'),
            Box::new(|c| c == 'o'),
            Box::new(|c| c != 'g'),
        ];
        assert!(all_predicts(&mut v, "foo"));
    }

    #[test]
    fn test_alphanum_minimum_length() {
        assert_eq!(Alphanum.minimum_length(), 3);
    }

    #[test]
    fn test_alphanum_verify_pass() {
        assert!(Alphanum.verify("fooBar00"));
        assert!(Alphanum.verify("0aA"));
    }

    #[test]
    fn test_alphanum_verify_fail() {
        assert!(!Alphanum.verify("foobar00"));
        assert!(!Alphanum.verify("FOOBAR00"));
        assert!(!Alphanum.verify("FOObarBaZ"));
        assert!(!Alphanum.verify("0123456"));
        assert!(!Alphanum.verify("foo!bar&"));
    }

    #[test]
    fn test_alphanumsymbol_minimum_length() {
        assert_eq!(AlphanumSymbol.minimum_length(), 4);
    }

    #[test]
    fn test_alphanumsymbol_verify_pass() {
        assert!(AlphanumSymbol.verify("fooBar00!@"));
        assert!(AlphanumSymbol.verify("0aA#"));
    }

    #[test]
    fn test_alphanumsymbol_verify_fail() {
        assert!(!AlphanumSymbol.verify("fooBAR00"));
        assert!(!AlphanumSymbol.verify("FOOBAR00*()"));
        assert!(!AlphanumSymbol.verify("FOObarBaZ{}"));
        assert!(!AlphanumSymbol.verify("0123456"));
        assert!(!AlphanumSymbol.verify("foobar00"));
        assert!(!AlphanumSymbol.verify("~!@#$%"));
    }
}
