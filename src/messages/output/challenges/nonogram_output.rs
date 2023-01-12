use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NonogramSolverOutput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}
