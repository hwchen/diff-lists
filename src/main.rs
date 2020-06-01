//! Takes two lists (Currently only newline delimited), and reports which members are in one list
//! but not the other.
//!
//! Does not require that the lists have unique members.

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;

fn main() -> Result<()> {
    let list_1_path = std::env::args().nth(1).context("Missing list 1")?;
    let list_2_path = std::env::args().nth(2).context("Missing list 2")?;

    let list_1 = fs::read_to_string(list_1_path).context("Could not read list 1")?;
    let list_2 = fs::read_to_string(list_2_path).context("Could not read list 2")?;

    let set_1: HashSet<_> = list_1.lines().collect();
    let set_2: HashSet<_> = list_2.lines().collect();

    let only_in_set_1 = set_1.difference(&set_2);
    let only_in_set_2 = set_2.difference(&set_1);

    println!("Only in list 1: {:?}", only_in_set_1);
    println!("Only in list 2: {:?}", only_in_set_2);

    Ok(())
}
