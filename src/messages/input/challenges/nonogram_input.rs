use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NonogramSolverInput {
    pub grid: String,
}