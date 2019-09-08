pub struct State {
    pub audio_active: bool,
    pub speed: f64 
}

impl State {
    pub fn new() -> Self {
        Self {
            audio_active: true, 
            speed: 0.5 
        }
    }
}