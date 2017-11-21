// TODO Maybe: Move programstate.rs into here
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

pub const PORTION_DEBUG: f32 = 0.35;
pub const FONT_SIZE: u32 = 32;
pub const NUM_LINES_ON_SCREEN: usize = 25;
pub const LINE_HEIGHT: f32 = 2.0/NUM_LINES_ON_SCREEN as f32;
pub const TEXT_HEIGHT: f32 = 0.618 * LINE_HEIGHT; // 1/golden ratio for aesthetic reasons
pub const NUM_CHARS_PER_LINE: u32 = 40;
pub const CHAR_WIDTH: f32 = 2.0*PORTION_DEBUG/NUM_CHARS_PER_LINE as f32;

pub const MAX_DEBUG_BUFFER_SIZE: usize = 1000;

pub fn prompt_for_val(prompt: &str) -> String {
    print!("{}", prompt);

    let mut input = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);

    input.lines().last().unwrap().trim().to_string()
}

pub fn string_to_u16(s: &str) -> Result<u16, <u16 as FromStr>::Err> {
	if s.len() == 0 {return Ok(0);}
	let base = if s.len() >= 2 && s[..2].to_lowercase() == "0x" {16} else {10};
	u16::from_str_radix(if base == 10 {s} else {&s[2..]}, base)
}

pub fn max(lhs: usize, rhs: usize) -> usize {
	if rhs > lhs {rhs} else {lhs}
}

pub fn min(lhs: usize, rhs: usize) -> usize {
	if rhs < lhs {rhs} else {lhs}
}