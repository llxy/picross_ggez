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

    // pub fn row_hints(&self) -> &[Hint] {
    //     self.matrix.rows()
    // }
}
