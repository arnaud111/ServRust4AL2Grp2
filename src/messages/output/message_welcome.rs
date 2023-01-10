use serde::Serialize;

#[derive(Serialize)]
pub struct Welcome {
    pub version: u8,
}

impl Welcome {
    pub fn clone(&self) -> Welcome {
        Welcome {
            version: self.version
        }
    }
}
