use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::filter::Filter;
use crate::mutate;
use crate::operator::Operator;

const SIZE: usize = 64;
const COLUMN_SIZES: [usize; 3] = [2, 4, 8];
const FREQ_MIN: f32 = 20.0;
const FREQ_MAX: f32 = 120.0;

#[derive(Debug, Clone)]
pub struct Synth {
    rng: SmallRng,
    sample_rate: f32,
    tempo: u64,
    update: u64,
    frequency: f32,
    groups: usize,
    operators: [Operator; SIZE],
    filter: Filter,
}

impl Synth {
    pub fn new(seed: u64, tempo: u64, sample_rate: f32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let update = Self::update(&mut rng, sample_rate, tempo);
        let frequency = rng.gen_range(FREQ_MIN, FREQ_MAX);
        let groups = Self::groups(&mut rng);

        let mut operators = [Operator::new(sample_rate, frequency); SIZE];
        for operator in &mut operators {
            operator.mutate(&mut rng);
        }

        let filter = Filter::new(&mut rng);

        Synth {
            rng,
            sample_rate,
            tempo,
            update,
            frequency,
            groups,
            operators,
            filter,
        }
    }

    fn update(rng: &mut SmallRng, sample_rate: f32, tempo: u64) -> u64 {
        let bpm = (sample_rate as u64 * 60) / tempo;
        rng.gen_range(bpm / 2, bpm * 2)
    }

    fn groups(rng: &mut SmallRng) -> usize {
        let groups = *COLUMN_SIZES.choose(rng).unwrap_or(&COLUMN_SIZES[0]);
        println!("LAYOUT: {}x{}", groups, SIZE / groups);
        groups
    }
}

impl Iterator for Synth {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.update -= 1;
        if self.update == 0 {
            match self.rng.gen_range(0.0, 1.0) {
                r if r > 0.2 => {
                    // Mutate an operator
                    if let Some(operator) = self.operators.choose_mut(&mut self.rng) {
                        operator.mutate(&mut self.rng);
                        println!("{:?}", operator);
                    }
                }
                r if r < 0.05 => {
                    // Mutate frequency
                    self.frequency = mutate!(self.rng, self.frequency, FREQ_MIN, FREQ_MAX);
                    for operator in &mut self.operators {
                        operator.set_frequency(self.frequency);
                    }
                    println!("{}Hz", self.frequency);
                }
                _ => {
                    // Mutate filter
                    self.filter.mutate(&mut self.rng);
                    println!("{:?}", self.filter);
                }
            }
            self.update = Self::update(&mut self.rng, self.sample_rate, self.tempo)
        }

        let value = self
            .operators
            .chunks_mut(self.groups)
            .fold(0.0, |value, operators| {
                operators.iter_mut().fold(1.0, |column_value, operator| {
                    operator.generate(column_value).unwrap_or_else(|| {
                        operator.restart();
                        column_value
                    })
                }) + value
            });
        Some(self.filter.generate(value))
    }
}
