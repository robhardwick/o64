use std::f32::consts::PI;

use rand::rngs::SmallRng;
use rand::Rng;

use crate::bool2f32;
use crate::envelope::Envelope;

const TAU: f32 = PI * 2.0;
const SQUARE_WAVETABLE: [f32; 2] = [-1.0, 1.0];
const SAW_DIRECTION: f32 = 1.0; // Rising

#[derive(Debug, Clone, Copy)]
pub enum OperatorMode {
    Ratio,
    Fixed,
}

#[derive(Debug, Clone, Copy)]
pub enum OperatorWaveform {
    Sine,
    Square,
    Saw,
}

#[derive(Debug, Clone, Copy)]
pub struct Operator {
    sample_rate: f32,
    key_frequency: f32,
    coarse: f32,
    fine: f32,
    mode: OperatorMode,
    waveform: OperatorWaveform,
    envelope: Envelope,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    state: f32,
}

impl Operator {
    pub fn new(sample_rate: f32, key_frequency: f32) -> Self {
        let frequency = Self::frequency(sample_rate, key_frequency, OperatorMode::Ratio, 1.0, 1.0);
        Operator {
            sample_rate,
            key_frequency,
            coarse: 1.0,
            fine: 1.0,
            mode: OperatorMode::Ratio,
            waveform: OperatorWaveform::Sine,
            envelope: Envelope::new(sample_rate),
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            state: 0.0,
        }
    }

    pub fn restart(&mut self) {
        self.envelope.restart();
    }

    pub fn mutate(&mut self, rng: &mut SmallRng) {
        if rng.gen_bool(0.2) {
            self.envelope.mutate(rng);
        } else {
            if rng.gen_bool(0.5) {
                self.mode = OperatorMode::Ratio;
                self.coarse = rng.gen_range(0.99, 1.0);
            } else {
                self.mode = OperatorMode::Fixed;
                self.coarse = if rng.gen_bool(0.5) {
                    rng.gen_range(0.01, 1.0)
                } else {
                    self.key_frequency
                }
            }
            self.waveform = match rng.gen_range(0, 3) {
                0 => OperatorWaveform::Sine,
                1 => OperatorWaveform::Square,
                _ => OperatorWaveform::Saw,
            };
            self.fine = rng.gen_range(0.1, 1.0);
            self.frequency = Self::frequency(
                self.sample_rate,
                self.key_frequency,
                self.mode,
                self.coarse,
                self.fine,
            );
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.key_frequency = frequency;
        self.frequency = Self::frequency(
            self.sample_rate,
            self.key_frequency,
            self.mode,
            self.coarse,
            self.fine,
        );
    }

    #[inline(always)]
    pub fn generate(&mut self, input: f32) -> Option<f32> {
        match self.waveform {
            OperatorWaveform::Sine => {
                self.phase += TAU * self.frequency;
                self.phase +=
                    (bool2f32!(self.phase >= TAU) * -TAU) + (bool2f32!(self.phase < 0.0) * TAU);
                self.state = self.phase.sin() * input * self.amplitude;
            }
            OperatorWaveform::Square => {
                self.phase += self.frequency;
                self.phase +=
                    (bool2f32!(self.phase >= 1.0) * -1.0) + (bool2f32!(self.phase < 0.0) * 1.0);
                self.state = SQUARE_WAVETABLE[(self.phase < 0.5) as usize] * self.amplitude;
            }
            OperatorWaveform::Saw => {
                self.phase += SAW_DIRECTION * self.frequency;
                self.phase +=
                    (bool2f32!(self.phase > 1.0) * -1.0) + (bool2f32!(self.phase < 0.0) * 1.0);
                self.state = ((self.phase * 2.0) - 1.0) * self.amplitude;
            }
        }

        self.envelope
            .next()
            .map(|amplitude| self.state * amplitude * input)
    }

    fn frequency(
        sample_rate: f32,
        key_frequency: f32,
        mode: OperatorMode,
        coarse: f32,
        fine: f32,
    ) -> f32 {
        let operator_frequency = coarse + (fine * 0.001);
        let frequency = match mode {
            OperatorMode::Ratio => key_frequency * operator_frequency,
            OperatorMode::Fixed => operator_frequency,
        };
        frequency / sample_rate
    }
}
