use rand::rngs::SmallRng;
use rand::Rng;

use crate::mutate;

const MIN_GAIN: f32 = 0.9;
const MAX_GAIN: f32 = 1.0;
const MIN_CUTOFF: f32 = 0.2;
const MAX_CUTOFF: f32 = 0.9;
const MIN_RESONANCE: f32 = 0.01;
const MAX_RESONANCE: f32 = 0.7;
const MIN_FEEDBACK: f32 = 0.01;
const MAX_FEEDBACK: f32 = 0.7;

#[derive(Debug, Clone)]
pub struct Filter {
    gain: f32,
    cutoff: f32,
    resonance: f32,
    filter_feedback: f32,
    feedback: f32,
    buffer: [f32; 4],
}

impl Filter {
    pub fn new(rng: &mut SmallRng) -> Self {
        let gain = rng.gen_range(MIN_GAIN, MAX_GAIN);
        let cutoff = rng.gen_range(MIN_CUTOFF, MAX_CUTOFF);
        let resonance = rng.gen_range(MIN_RESONANCE, MAX_RESONANCE);
        let filter_feedback = rng.gen_range(MIN_FEEDBACK, MAX_FEEDBACK);
        Filter {
            gain,
            cutoff,
            resonance,
            filter_feedback,
            feedback: Self::feedback(cutoff, resonance) * filter_feedback,
            buffer: [0.; 4],
        }
    }

    pub fn mutate(&mut self, rng: &mut SmallRng) {
        self.gain = mutate!(rng, self.gain, MIN_GAIN, MAX_GAIN);
        self.cutoff = mutate!(rng, self.cutoff, MIN_CUTOFF, MAX_CUTOFF);
        self.resonance = mutate!(rng, self.resonance, MIN_RESONANCE, MAX_RESONANCE);
        self.filter_feedback = mutate!(rng, self.filter_feedback, MIN_FEEDBACK, MAX_FEEDBACK);
        self.feedback = Self::feedback(self.cutoff, self.resonance) * self.filter_feedback;
    }

    pub fn generate(&mut self, sample: f32) -> f32 {
        self.buffer[0] += self.cutoff
            * (sample - self.buffer[0] + self.feedback * (self.buffer[0] - self.buffer[1]));
        for index in 1..=3 {
            self.buffer[index] += self.cutoff * (self.buffer[index - 1] - self.buffer[index]);
        }
        self.buffer[3]
    }

    fn feedback(resonance: f32, cutoff: f32) -> f32 {
        resonance + resonance / (1.0 - cutoff)
    }
}
