use std::str::FromStr;

use num::Complex;

pub fn parse_pair<T: FromStr>(s: &str, separator: &str) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ",") {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}
