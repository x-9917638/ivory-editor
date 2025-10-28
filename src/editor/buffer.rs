#[derive(Default)]
pub struct Buffer {
    pub text: Vec<String>
}

impl Buffer {
    pub fn append(&mut self, s: &str) {
        self.text.push(s.to_owned());
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}