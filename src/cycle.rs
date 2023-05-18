use esp_idf_svc::nvs::{EspNvs, NvsPartitionId};

pub struct Cycle<'a, T, P: NvsPartitionId> {
    current_index: u32,
    options: Vec<T>,
    nvs: &'a EspNvs<P>,
    namespace: &'a str,
}

impl<'a, T, P: NvsPartitionId> Cycle<'a, T, P> {
    #[allow(dead_code)]
    pub fn new(nvs: &'a EspNvs<P>, options: Vec<T>, namespace: &'a str) -> Self {
        let current_index = match nvs.get_u32(namespace) {
            Ok(index) => match index {
                Some(index) => index,
                None => 0,
            },
            Err(_) => 0,
        };
        Self {
            namespace,
            nvs,
            current_index,
            options,
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.options.len() as u32;
        self.nvs
            .set_u32(self.namespace, self.current_index)
            .unwrap();
    }

    #[allow(dead_code)]
    pub fn get_current(&self) -> &T {
        &self.options[self.current_index as usize]
    }
}
