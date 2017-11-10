// TODO Maybe: Move programstate.rs into here
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

// Is this the best place for these?
pub const PORTION_DEBUG: f32 = 0.35;
pub const FONT_SIZE: u32 = 32;
pub const NUM_LINES_ON_SCREEN: usize = 25;
pub const LINE_HEIGHT: f32 = 2.0/NUM_LINES_ON_SCREEN as f32;
pub const TEXT_HEIGHT: f32 = 0.618 * LINE_HEIGHT; // 1/golden ratio for aesthetic reasons
pub const NUM_CHARS_PER_LINE: u32 = 40;
pub const CHAR_WIDTH: f32 = 2.0*PORTION_DEBUG/NUM_CHARS_PER_LINE as f32;

pub fn prompt_for_val<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{}", prompt);

    let mut input = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);

    input.lines().last().unwrap().trim().parse()
}

pub fn max(lhs: usize, rhs: usize) -> usize {
	if rhs > lhs {rhs} else {lhs}
}

pub fn min(lhs: usize, rhs: usize) -> usize {
	if rhs < lhs {rhs} else {lhs}
}