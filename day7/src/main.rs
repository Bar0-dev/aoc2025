use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

enum Part {
    Source(Coords),
    Void(Coords),
    Splitter(Coords),
    Other(()),
}

struct Manifold {
    segments: Vec<Vec<Part>>,
}

impl Manifold {
    fn new(segments_raw: Vec<&str>) -> Self {
        let segments = segments_raw
            .iter()
            .enumerate()
            .map(|(y, segment)| {
                segment
                    .chars()
                    .enumerate()
                    .map(|(x, part)| match part {
                        '.' => Part::Void(Coords { x, y }),
                        '^' => Part::Splitter(Coords { x, y }),
                        'S' => Part::Source(Coords { x, y }),
                        _ => Part::Other(()),
                    })
                    .collect::<Vec<Part>>()
            })
            .collect::<Vec<Vec<Part>>>();
        Self { segments }
    }
}

#[derive(Clone)]
struct QuantumBeam {
    beam_history: HashMap<Coords, usize>,
    split_count: usize,
}

impl QuantumBeam {
    fn new() -> Self {
        Self {
            beam_history: HashMap::new(),
            split_count: 0,
        }
    }

    fn spawn_quantum_beams(&mut self, position: &Coords, beams_count: usize) {
        *self.beam_history.entry(*position).or_insert(0) += beams_count;
    }

    fn split_quantum_beams(&mut self, splitter: &Coords, beams_count: usize) {
        let split_left = Coords {
            x: splitter.x.saturating_sub(1),
            y: splitter.y,
        };
        let split_right = Coords {
            x: splitter.x.saturating_add(1),
            y: splitter.y,
        };
        self.spawn_quantum_beams(&split_left, beams_count);
        self.spawn_quantum_beams(&split_right, beams_count);
    }

    fn simulate_quantum_beam_in_segment(&mut self, segment: &[Part]) {
        for part in segment.iter() {
            match part {
                Part::Source(source) => self.spawn_quantum_beams(source, 1),
                Part::Void(void) => {
                    let above_void = Coords {
                        x: void.x,
                        y: void.y.saturating_sub(1),
                    };
                    let beam_superposition_count = self.beam_history.get(&above_void).unwrap_or(&0);
                    self.spawn_quantum_beams(void, *beam_superposition_count);
                }
                Part::Splitter(splitter) => {
                    let above_splitter = Coords {
                        x: splitter.x,
                        y: splitter.y.saturating_sub(1),
                    };
                    let beam_superposition_count =
                        self.beam_history.get(&above_splitter).unwrap_or(&0);

                    if *beam_superposition_count > 0 {
                        self.split_count += 1;
                    }
                    self.split_quantum_beams(splitter, *beam_superposition_count);
                }
                Part::Other(_) => (),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input";
    let file = fs::read_to_string(path)?;
    let segments = file.lines().collect();
    let manifold = Manifold::new(segments);

    let mut quantum_beam = QuantumBeam::new();
    for segment in &manifold.segments {
        quantum_beam.simulate_quantum_beam_in_segment(segment);
    }
    println!("part 1: {}", quantum_beam.split_count);
    let timelines = manifold
        .segments
        .last()
        .expect("Vector is empty")
        .iter()
        .filter_map(|part| match part {
            Part::Void(void) => quantum_beam.beam_history.get(void).copied(),
            _ => None,
        })
        .sum::<usize>();

    println!("part 2: {}", timelines);

    Ok(())
}
