use std::error::Error;
use std::fs;

enum Parts {
    Source,
    Void,
    Splitter,
    Beam,
    Other(char),
}

struct Manifold {
    segments: Vec<Vec<(usize, Parts)>>,
    beam_positions: Vec<usize>,
    split_count: usize,
    parts_in_segment: usize,
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
                        _ => (position, Parts::Other(part)),
                    })
                    .collect::<Vec<(usize, Parts)>>()
            })
            .collect::<Vec<Vec<(usize, Parts)>>>();
        Self {
            segments,
            beam_positions: Vec::new(),
            split_count: 0,
            parts_in_segment: segments_raw[0].len(),
        }
    }

    fn spawn_beam(&mut self, position: usize) {
        self.beam_positions.push(position);
    }

    // fn propagate_in_void(&mut self, position: usize) {
    //     if self.beam_positions.contains(position) {
    //
    //     }
    //     self.beam_positions.push(position);
    // }
    //
    fn split_beam(&mut self, position: usize) {
        let pos_before = position.saturating_sub(1);
        let pos_after = position.saturating_add(1).max(self.parts_in_segment);
        let position_idx = self.beam_positions.iter().position(|p| p == position);
        self.beam_positions.remove(postion)
    }

    fn simulate_beam_in_segment(&self, segment: &Vec<(usize, Parts)>) {
        segment.iter().map(|(position, part)| match part {
            Parts::Source => self.spawn_beam(*position),
            Parts::Void => (),
            Parts::Splitter => self.split_beam(),
            Parts::Beam => (),
            Parts::Other('0') => (),
        });
    }

    fn simulate(&self) {
        self.segments
            .iter()
            .for_each(|segment| self.simulate_beam_in_segment(segment));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "test";
    let file = fs::read_to_string(path)?;
    let segments = file.lines().collect();
    let manifold = Manifold::new(segments);

    Ok(())
    // let beam = Beam::new(origin);
}
