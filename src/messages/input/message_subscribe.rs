use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subscribe {
    pub name: String
}