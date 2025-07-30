pub struct Semitone(pub u8);

impl Semitone {
    pub fn get_frequency(&self) -> f32 {
        27.5 * 2f32.powf((self.0 as f32 - 21.0) / 12.0)
    }
}
