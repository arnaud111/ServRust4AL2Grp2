use serde::Serialize;

#[derive(Serialize)]
pub struct NonogramSolverOutput {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}

impl NonogramSolverOutput {
    pub fn clone(&self) -> NonogramSolverOutput {
        NonogramSolverOutput {
            rows: self.rows.clone(),
            cols: self.cols.clone()
        }
    }
}
