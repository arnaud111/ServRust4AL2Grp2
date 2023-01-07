use serde::Deserialize;

#[derive(Deserialize)]
pub struct NonogramSolverInput {
    pub grid: String,
}