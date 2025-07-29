use crate::envelope::EnvelopeADSR;

pub trait InstrumentTrait {
    fn get_volume(&self) -> f32;
    fn set_volume(&mut self, volume: f32);
    fn get_envelope_mut(&self) -> &EnvelopeADSR;
    fn base_sound(&self, f: f32, t: f32) -> f32;

    fn make_sound(&self, f: f32, t: f32) -> f32 {
        self.base_sound(f, t) * self.get_envelope_mut().get_amplitude(t)
    }
}
