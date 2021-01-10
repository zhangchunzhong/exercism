
pub struct PascalsTriangle {
    rows: u32
}

fn make_next_row(v: &Vec<u32>) -> Vec<u32> {
    let mut rv = Vec::new();
    if v.len() == 0 {
        rv.push(1);
        rv
    } else if v.len() == 1 {
        rv.push(1);
        rv.push(1);
        rv
    } else {
        let mut pv = 0;
        rv.push(1);
        for i in v {
            if pv == 0 {
                pv = *i;
            } else {
                rv.push(pv+(*i));
                pv = *i;
            }
        }
        rv.push(1);
        rv
    }
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rv = Vec::<Vec<u32>>::new();
        let mut pv = Vec::new();
        for _ in 0..self.rows {
            let mut mv = Vec::new();
            mv.extend_from_slice(&make_next_row(&pv));
            pv.truncate(0);
            pv.extend_from_slice(&mv);
            rv.push(mv);
            println!("");
        }
        rv
    }
}
