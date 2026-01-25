use std::io::{BufRead, Write, stdin, stdout};

#[derive(Debug)]
pub struct Params {
    params: Vec<(String, String)>,
}
impl Params {
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    pub fn from_stdin() -> Self {
        let mut params = Params::new();
        let mut stdin = stdin().lock();
        loop {
            let mut buf = String::new();

            stdin
                .read_line(&mut buf)
                .expect("Failed to read line from stdin");
            let buf = buf.trim();
            if buf.is_empty() {
                return params;
            }
            if let Some((key, value)) = buf.split_once('=') {
                params.insert(key.to_string(), value.to_string());
            }
        }
    }
    pub fn write_to_stdout(self) -> std::io::Result<()> {
        let mut stdout = stdout().lock();
        for (key, value) in self.params.iter() {
            if key != "" && value != "" {
                writeln!(stdout, "{}={}", key, value)?;
            }
        }
        writeln!(stdout, "")?;
        Ok(())
    }
    pub fn insert(&mut self, key: String, value: String) {
        if let Some(e) = self.params.iter_mut().find(|e| e.0 == key) {
            e.1 = value;
        } else {
            self.params.push((key, value));
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|e| e.0 == key)
            .map(|e| e.1.as_str())
    }
}
