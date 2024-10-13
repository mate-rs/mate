#[derive(Debug)]
pub struct Job {
    pub data: String,
}

impl Job {
    pub fn dispatch(&self) {
        println!("{}", self.data);
    }
}
