use std::error::Error;
use std::fs;

enum Parts {
    Source,
    Void,
    Splitter,
    Other(()),
}

struct Manifold {
    segments: Vec<Vec<(usize, Parts)>>,
}

impl Manifold {
    fn new(segments_raw: Vec<&str>) -> Self {
        let segments = segments_raw
            .iter()
            .map(|segment| {
                segment
                    .chars()
                    .enumerate()
                    .map(|(position, part)| match part {
                        '.' => (position, Parts::Void),
                        '^' => (position, Parts::Splitter),
                        'S' => (position, Parts::Source),
                        _ => (position, Parts::Other(())),
                    })
                    .collect::<Vec<(usize, Parts)>>()
            })
            .collect::<Vec<Vec<(usize, Parts)>>>();
        Self { segments }
    }
}

struct Beam {
    beam_positions: Vec<usize>,
    split_count: usize,
}

impl Beam {
    fn new() -> Self {
        Self {
            beam_positions: Vec::new(),
            split_count: 0,
        }
    }

    fn spawn_beam(beam_positions: &mut Vec<usize>, position: usize) {
        if !beam_positions.contains(&position) {
            beam_positions.push(position);
        }
    }

    fn split_beam(beam_positions: &mut Vec<usize>, splitter_position: usize) -> bool {
        let split_beam = beam_positions.contains(&splitter_position);
        if split_beam {
            let split_left = splitter_position.saturating_sub(1);
            let split_right = splitter_position.saturating_add(1);
            let beam_position_idx = beam_positions
                .iter()
                .position(|position| *position == splitter_position)
                .expect("No index found");
            beam_positions.remove(beam_position_idx);
            Self::spawn_beam(beam_positions, split_left);
            Self::spawn_beam(beam_positions, split_right);
        }
        split_beam
    }

    fn simulate_beam_in_segment(&mut self, segment: &[(usize, Parts)]) {
        for (position, part) in segment.iter() {
            match part {
                Parts::Source => Self::spawn_beam(&mut self.beam_positions, *position),
                Parts::Void => (),
                Parts::Splitter => {
                    let splitted = Self::split_beam(&mut self.beam_positions, *position);
                    if splitted {
                        self.split_count += 1
                    }
                }
                Parts::Other(_) => (),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input";
    let file = fs::read_to_string(path)?;
    let segments = file.lines().collect();
    let manifold = Manifold::new(segments);
    let mut beam = Beam::new();
    for segment in manifold.segments {
        beam.simulate_beam_in_segment(&segment);
    }

    println!("part 1: {}", beam.split_count);

    Ok(())
    // let beam = Beam::new(origin);
}
