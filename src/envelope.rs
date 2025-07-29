#[derive(PartialEq)]
enum EnvelopeState {
    Idle,
    TriggeredOn(f32),  // Time when the envelope was triggered on
    TriggeredOff(f32), // Time when the envelope was triggered off
}

pub struct EnvelopeADSR {
    attack_time: f32,
    decay_time: f32,
    release_time: f32,

    attack_amplitude: f32,
    sustain_amplitude: f32,

    state: EnvelopeState,
}

impl EnvelopeADSR {
    pub fn new(
        attack_time: f32,
        decay_time: f32,
        release_time: f32,
        attack_amplitude: f32,
        sustain_amplitude: f32,
    ) -> Self {
        EnvelopeADSR {
            attack_time,
            decay_time,
            release_time,
            attack_amplitude,
            sustain_amplitude,
            state: EnvelopeState::Idle,
        }
    }

    pub fn is_idle(&self) -> bool {
        self.state == EnvelopeState::Idle
    }

    pub fn trigger_on(&mut self, current_time: f32) {
        self.state = EnvelopeState::TriggeredOn(current_time);
    }

    pub fn trigger_off(&mut self, current_time: f32) {
        if matches!(self.state, EnvelopeState::TriggeredOn(_)) {
            self.state = EnvelopeState::TriggeredOff(current_time);
        }
    }

    pub fn reset(&mut self) {
        self.state = EnvelopeState::Idle;
    }

    pub fn can_be_reset(&self, current_time: f32) -> bool {
        if let EnvelopeState::TriggeredOff(time_off) = self.state {
            let elapsed = current_time - time_off;
            return elapsed >= self.release_time;
        }
        false
    }

    pub fn get_amplitude(&self, current_time: f32) -> f32 {
        let amplitude = match self.state {
            EnvelopeState::Idle => 0.0,
            EnvelopeState::TriggeredOn(time_on) => {
                let elapsed = current_time - time_on;

                if elapsed <= self.attack_time {
                    // Attack phase
                    elapsed / self.attack_time * self.attack_amplitude
                } else if elapsed <= self.attack_time + self.decay_time {
                    // Decay phase
                    self.attack_amplitude
                        + (elapsed - self.attack_time) / self.decay_time
                            * (self.sustain_amplitude - self.attack_amplitude)
                } else {
                    // Sustain phase
                    self.sustain_amplitude
                }
            }
            EnvelopeState::TriggeredOff(time_off) => {
                // Release phase
                let elapsed = current_time - time_off;

                (1.0 - elapsed / self.release_time) * self.sustain_amplitude
            }
        };

        if amplitude < 0.0001 {
            0.0
        } else {
            amplitude
        }
    }
}
