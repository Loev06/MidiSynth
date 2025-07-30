use crate::instrument::InstrumentTrait;
use crate::semitone::Semitone;

pub struct Note<I: InstrumentTrait> {
    instrument: I,
    semitone: Semitone,
    start_time: f32,
    end_time: Option<f32>,
}

impl<I: InstrumentTrait> Note<I> {
    pub fn new(semitone: Semitone, start_time: f32, end_time: Option<f32>) -> Self {
        Note {
            instrument: I::default(),
            semitone,
            start_time,
            end_time,
        }
    }

    pub fn next_sample(&mut self, t: f32) -> f32 {
        if t < self.start_time {
            return 0.0;
        }

        if let Some(end_time) = self.end_time {
            if t >= end_time {
                self.instrument.trigger_off(t);
            }
        }

        let frequency = self.semitone.get_frequency();
        self.instrument.make_sound(frequency, t)
    }
}
