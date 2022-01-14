use std::str::FromStr;

pub struct Message {
    pub parameter: Vec<String>,
}
impl Message {
    pub fn from_strings(strings: Vec<String>) -> Self {
        Self { parameter: strings }
    }
    pub fn get_value<T: FromStr>(&self, index: usize) -> Result<T, ()> {
        if self.parameter.len() > index {
            match self.parameter[index].parse::<T>() {
                Ok(p) => Ok(p),
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }
}
