use nalgebra::DMatrix;

type PuzzleMatrix = DMatrix<bool>;
// type Hint = &[i8];

#[derive(Debug)]
pub struct Puzzle {
    pub matrix: PuzzleMatrix,
}

impl Puzzle {
    pub fn rand_new() -> Self {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        Self {
            matrix: DMatrix::from_fn(5, 5, |_r, _c| rng.gen_bool(0.5)),
        }
    }

    pub fn check(&self, solution: Vec<Vec<bool>>) -> bool {
        let fsol = solution.into_iter().flatten().collect::<Vec<bool>>();
        let sol_mat = DMatrix::from_vec(5, 5, fsol);

        self.matrix.eq(&sol_mat)
    }

    pub fn row_hints(&self) -> Vec<String> {
        let mut res = vec![];
        for row in self.matrix.row_iter() {
            let (mut hint, acc) = row
                .iter()
                .fold(("".to_string(), 0), |(hint_str, acc), piece| {
                    if *piece {
                        (hint_str, acc + 1)
                    } else {
                        let new_hint_str = if acc > 0 {
                            hint_str + " " + &(acc.to_string())
                        } else {
                            hint_str
                        };
                        (new_hint_str, 0)
                    }
                });

            if acc > 0 {
                hint.push_str(&(acc.to_string()))
            }

            res.push(hint);
        }

        res
    }

    pub fn col_hints(&self) -> Vec<String> {
        let mut res = vec![];
        for col in self.matrix.column_iter() {
            let (mut hint, acc) = col
                .iter()
                .fold(("".to_string(), 0), |(hint_str, acc), piece| {
                    if *piece {
                        (hint_str, acc + 1)
                    } else {
                        let new_hint_str = if acc > 0 {
                            hint_str + &(acc.to_string()) + "\n"
                        } else {
                            hint_str
                        };
                        (new_hint_str, 0)
                    }
                });

            if acc > 0 {
                hint.push_str(&(acc.to_string()))
            }

            res.push(hint);
        }

        res
    }
}
