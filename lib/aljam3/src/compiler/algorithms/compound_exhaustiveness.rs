// Compound Condition Partition Refinement Algorithm

use std::collections::HashSet;

pub struct GridCell {
    pub id: usize,
    pub covered_by_branches: HashSet<usize>,
}

pub struct CartesianGrid {
    pub cells: Vec<GridCell>,
    pub num_variables: usize,
}

impl CartesianGrid {
    pub fn new(total_cells: usize, num_variables: usize) -> Self {
        let mut cells = Vec::with_capacity(total_cells);
        for i in 0..total_cells {
            cells.push(GridCell {
                id: i,
                covered_by_branches: HashSet::new(),
            });
        }
        Self { cells, num_variables }
    }

    pub fn mark_coverage(&mut self, branch_id: usize, cell_indices: &[usize]) {
        for &idx in cell_indices {
            if idx < self.cells.len() {
                self.cells[idx].covered_by_branches.insert(branch_id);
            }
        }
    }

    /// Checks if any cell is covered by more than one branch (PGE06005)
    pub fn check_overlap(&self) -> Option<(usize, usize, usize)> {
        for cell in &self.cells {
            if cell.covered_by_branches.len() > 1 {
                let mut iter = cell.covered_by_branches.iter();
                let branch1 = *iter.next().unwrap();
                let branch2 = *iter.next().unwrap();
                return Some((branch1, branch2, cell.id));
            }
        }
        None
    }

    /// Checks if all cells are covered by at least one branch (PGE06008)
    pub fn check_exhaustiveness(&self) -> Result<(), Vec<usize>> {
        let mut uncovered = Vec::new();
        for cell in &self.cells {
            if cell.covered_by_branches.is_empty() {
                uncovered.push(cell.id);
            }
        }
        
        if uncovered.is_empty() {
            Ok(())
        } else {
            Err(uncovered)
        }
    }
}
