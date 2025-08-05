pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let row_count: usize = row_count as usize;
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count);
        
        for i in 0..row_count {
            let mut row: Vec<u32> = vec![1_u32; i + 1];

            if i > 0 {
                for j in 1..i {
                    row[j] = rows[i - 1][j - 1] + rows[i - 1][j];
                }
            }

            rows.push(row)
        }

        Self {
            rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
