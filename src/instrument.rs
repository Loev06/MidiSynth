pub mod bell;

use crate::envelope::EnvelopeADSR;

pub trait InstrumentTrait: Default {
    fn get_envelope_mut(&mut self) -> &mut EnvelopeADSR;
    fn base_sound(&self, f: f32, t: f32) -> f32;

    fn make_sound(&mut self, f: f32, t: f32) -> f32 {
        let base_sound = self.base_sound(f, t);
        let envelope = self.get_envelope_mut();

        if envelope.can_be_reset(t) {
            envelope.reset();
        }
        if envelope.is_idle() {
            envelope.trigger_on(t);
        }

        base_sound * envelope.get_amplitude(t)
    }

    fn trigger_off(&mut self, t: f32) {
        let envelope = self.get_envelope_mut();
        if envelope.is_idle() {
            envelope.trigger_off(t);
        }
    }
}
