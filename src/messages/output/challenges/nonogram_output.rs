use serde::Serialize;

#[derive(Serialize)]
pub struct NonogramSolverOutput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}
