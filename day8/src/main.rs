use anyhow::{Context, Result};
use std::fs::read_to_string;

struct Jbox {
    x: usize,
    y: usize,
    z: usize,
}

fn main() -> Result<()> {
    let path = "test";
    let file = read_to_string(path).context("Failed to read file to string")?;
    let lines = file.lines();
    let all_jboxes = lines
        .filter_map(|line: &str| -> Result<Jbox> {
            let coords = line
                .split(",")
                .map(|coord| coord.parse::<usize>().context("Invalid coordinate"));
            Jbox {
                x: coords.next().context("Missing x coorfinate")?,
                y: coords.next().context("Missing y coorfinate")?,
                z: coords.next().context("Missing z coorfinate")?,
            }
        })
        .collect::<Result<Vec<Jbox>>>()?;

    Ok(())
}
