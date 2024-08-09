#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_precision_loss)]

pub mod math;
pub mod parser;
pub mod physicsvalues;
#[cfg(test)]
mod tests;
