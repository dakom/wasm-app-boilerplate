pub struct State {
    pub text_input: String,
    pub results: Vec<String>
}

impl State {
    pub fn new() -> Self {
        Self {
            text_input: "".to_owned(),
            results: vec![]
        }
    }
}