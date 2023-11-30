#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod day1;

use std::{fmt::Debug, str::FromStr};

extern crate derive_more;
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct VecP<T, const P: &'static str = "\n">(Vec<T>);

impl<T: std::str::FromStr, const P: &'static str> FromStr for VecP<T, P> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(P)
                .filter_map(|v| {
                    let trimmed = v.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.parse())
                    }
                })
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}
