use rand::rngs::SmallRng;
use rand::Rng;

const RATIO_A: f32 = 0.3;
const RATIO_DR: f32 = 0.0001;

#[derive(Debug, Clone, Copy)]
enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}

#[derive(Debug, Clone, Copy)]
struct EnvelopeStateParams {
    coefficient: f32,
    base: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Envelope {
    sample_rate: f32,
    gain: f32,
    attack: EnvelopeStateParams,
    decay: EnvelopeStateParams,
    release: EnvelopeStateParams,
    sustain: f32,
    state: EnvelopeState,
    value: f32,
}

impl Envelope {
    pub fn new(sample_rate: f32) -> Self {
        Envelope {
            sample_rate,
            gain: 1.0,
            attack: EnvelopeStateParams::new(sample_rate, 4000.0, RATIO_A, 1.0 + RATIO_A),
            decay: EnvelopeStateParams::new(sample_rate, 40000.0, RATIO_DR, 0.8 - RATIO_DR),
            release: EnvelopeStateParams::new(sample_rate, 40000.0, RATIO_DR, -RATIO_DR),
            sustain: 1.0,
            state: EnvelopeState::Attack,
            value: 0.0,
        }
    }

    pub fn restart(&mut self) {
        self.state = EnvelopeState::Attack;
    }

    pub fn mutate(&mut self, rng: &mut SmallRng) {
        self.sustain = rng.gen_range(0.5, 1.0);
        self.attack = EnvelopeStateParams::new(
            self.sample_rate,
            rng.gen_range(1000.0, 8000.0),
            RATIO_A,
            1.0 + RATIO_A,
        );
        self.decay = EnvelopeStateParams::new(
            self.sample_rate,
            rng.gen_range(1000.0, 40000.0),
            RATIO_DR,
            self.sustain - RATIO_DR,
        );
        self.release = EnvelopeStateParams::new(
            self.sample_rate,
            rng.gen_range(8000.0, 40000.0),
            RATIO_DR,
            -RATIO_DR,
        );

        if rng.gen_bool(0.5) {
            self.state = EnvelopeState::Attack;
        } else {
            self.state = EnvelopeState::Release;
        }
    }
}

impl Iterator for Envelope {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            EnvelopeState::Off => return None,
            EnvelopeState::Attack => {
                self.value = self.attack.generate(self.value);
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.state = EnvelopeState::Decay;
                }
            }
            EnvelopeState::Decay => {
                self.value = self.decay.generate(self.value);
                if self.value <= self.sustain {
                    self.value = self.sustain;
                    self.state = EnvelopeState::Sustain;
                }
            }
            EnvelopeState::Sustain => (),
            EnvelopeState::Release => {
                self.value = self.release.generate(self.value);
                if self.value <= 0.0 {
                    self.value = 0.0;
                    self.state = EnvelopeState::Off;
                }
            }
        }
        Some(self.value * self.gain)
    }
}

impl EnvelopeStateParams {
    fn new(sample_rate: f32, rate: f32, ratio: f32, base: f32) -> Self {
        let rate = (rate / 1000.0) * sample_rate;
        let coefficient = if rate <= 0.0 {
            0.0
        } else {
            (-((1.0 + ratio) / ratio).ln() / rate).exp()
        };
        let base = base * (1.0 - coefficient);
        EnvelopeStateParams { coefficient, base }
    }

    fn generate(&self, value: f32) -> f32 {
        self.base + value * self.coefficient
    }
}
