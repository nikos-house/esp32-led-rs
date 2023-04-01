pub struct Cycle<T> {
    current_index: usize,
    options: Vec<T>,
}

impl<T: Clone> Cycle<T> {
    #[allow(dead_code)]
    pub fn new(options: Vec<T>) -> Self {
        let current_index = 0;
        Self {
            current_index,
            options,
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.options.len();
    }

    #[allow(dead_code)]
    pub fn get_current(&self) -> T {
        self.options[self.current_index].clone()
    }
}
