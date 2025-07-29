use crate::envelope::EnvelopeADSR;
use crate::instrument::InstrumentTrait;
use crate::source::{
    oscillator::{Oscillator, Waveform},
    SourceTrait,
};

pub struct Bell {
    envelope: EnvelopeADSR,
    oscillator: Oscillator,
    volume: f32,
}

impl Default for Bell {
    fn default() -> Self {
        Bell {
            envelope: EnvelopeADSR::new(0.01, 1.0, 0.0, 1.0, 0.0),
            oscillator: Oscillator::new(Waveform::SineSawtooth(2), 0.002, 1.0),
            volume: 1.0,
        }
    }
}

impl InstrumentTrait for Bell {
    fn get_envelope_mut(&mut self) -> &mut EnvelopeADSR {
        &mut self.envelope
    }

    fn base_sound(&self, f: f32, t: f32) -> f32 {
        self.volume * self.oscillator.next_sample(f, t)
    }
}
