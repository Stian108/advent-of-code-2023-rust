#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod day1;
pub mod day2;
pub mod day3;

use std::{fmt::Debug, str::FromStr};

extern crate derive_more;
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct VecP<T, const P: &'static str = "\n">(Vec<T>);

impl<T: std::str::FromStr, const P: &'static str> FromStr for VecP<T, P> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(P)
            .filter_map(|v| {
                let trimmed = v.trim();
                (!trimmed.is_empty()).then_some(trimmed.parse())
            })
            .collect::<Result<_, Self::Err>>()
            .map(Self)
    }
}
