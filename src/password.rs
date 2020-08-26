use std::fmt;
use std::iter;

use rand;
use rand::distributions::{Alphanumeric, Distribution};
use rand::{thread_rng, Rng};

pub struct Alphanum;

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

impl fmt::Display for Alphanum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "alphanum")
    }
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

fn lower_char(c: char) -> bool {
    c.is_lowercase()
}

fn upper_char(c: char) -> bool {
    c.is_uppercase()
}

fn number_char(c: char) -> bool {
    c.is_numeric()
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
    }
}
