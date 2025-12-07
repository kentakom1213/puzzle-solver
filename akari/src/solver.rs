//! ソルバの実装

use crate::{
    field::{Field, Solution, State},
    utility::{ADJ, GridUtility},
};

const MISMATCH_AKARI: &'static str = "The number of lights does not match.";
const OVERLAP_AKARI: &'static str = "The light is already in place.";
const UNLIT_CELL: &'static str = "There are cells that are not lighted.";

/// ソルバを表すトレイト
pub trait Solver {
    fn solve(&self, field: &Field) -> Option<Solution>;
    /// 解 sol が条件を満たすか判定
    fn check(field: &Field, sol: &Solution) -> Result<(), &'static str> {
        let (h, w) = (field.h, field.w);

        // あかりの配置が条件を満たすか判定
        for r in 0..field.h {
            for c in 0..field.w {
                let akari_count = (r, c).adj(h, w).filter(|&(r, c)| sol.field[r][c]).count();
                if match field.field[r][c] {
                    // あかりが置かれていれば x
                    State::Nil => sol.field[r][c],
                    State::Empty => false,
                    // あかりが置かれているか，周囲の数と一致しなければ x
                    State::Adj0 => sol.field[r][c] || akari_count != 0,
                    State::Adj1 => sol.field[r][c] || akari_count != 1,
                    State::Adj2 => sol.field[r][c] || akari_count != 2,
                    State::Adj3 => sol.field[r][c] || akari_count != 3,
                    State::Adj4 => sol.field[r][c] || akari_count != 4,
                } {
                    return Err(MISMATCH_AKARI);
                }
            }
        }

        // None: セルなし | ブロック
        // Some(false): あかりで照らされていない
        // Some(true): あかりで照らされている
        let mut fill: Vec<_> = field
            .field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| (c == &State::Empty).then_some(false))
                    .collect::<Vec<_>>()
            })
            .collect();

        // あかりが重複していないか判定
        for r in 0..h {
            for c in 0..w {
                if !sol.field[r][c] {
                    continue;
                }
                // その場を塗る
                fill[r][c].replace(true);
                for dir in ADJ {
                    // 特定方向に塗れるだけ塗る
                    for (nr, nc) in (r, c).while_dir(h, w, dir) {
                        // あかりが置かれていたら失敗
                        if sol.field[nr][nc] {
                            return Err(OVERLAP_AKARI);
                        }
                        // ブロックに当たったら終了
                        if fill[nr][nc].is_none() {
                            break;
                        }
                        fill[nr][nc].replace(true);
                    }
                }
            }
        }

        // すべてのセルが照らされているか
        if fill.into_iter().flatten().all(|c| c.unwrap_or(true)) {
            Ok(())
        } else {
            Err(UNLIT_CELL)
        }
    }
}

pub mod solvers {
    use crate::{
        field::{Field, Solution, State},
        solver::Solver,
        utility::{ADJ, GridUtility},
    };

    /// バックトラックによる愚直な求解
    pub struct Naive;

    impl Naive {
        fn dfs(
            field: &Field,
            pos: usize,
            sol: &Solution,
            fill: &Vec<Vec<Option<bool>>>,
            found: &mut Option<Solution>,
        ) {
            let (h, w) = (field.h, field.w);

            if found.is_some() {
                return;
            }

            // 最後のセルに来た場合，終了
            if pos == h * w {
                if Self::check(field, sol).is_ok() {
                    *found = Some(sol.clone());
                }
                return;
            }

            let (r, c) = (pos / w, pos % w);

            // あかりが設置できる場合
            if fill[r][c].is_some_and(|c| !c) {
                // あかりを設置
                let mut new_sol = sol.clone();
                new_sol.field[r][c] = true;

                // あかりが重複していないか判定
                let mut new_fill = fill.clone();
                // その場を塗る
                new_fill[r][c].replace(true);
                for dir in ADJ {
                    // 特定方向に塗れるだけ塗る
                    for (nr, nc) in (r, c).while_dir(h, w, dir) {
                        // あかりが置かれていたら失敗
                        if new_sol.field[nr][nc] {
                            return;
                        }
                        // ブロックに当たったら終了
                        if new_fill[nr][nc].is_none() {
                            break;
                        }
                        new_fill[nr][nc].replace(true);
                    }
                }

                // 再帰呼び出し
                Self::dfs(field, pos + 1, &mut new_sol, &mut new_fill, found);
            }

            // あかりを設置しない
            Self::dfs(field, pos + 1, sol, fill, found);
        }
    }

    impl Solver for Naive {
        fn solve(&self, field: &Field) -> Option<Solution> {
            let h = field.field.len();
            let w = field.field.get(0).as_ref().map(|r| r.len()).unwrap_or(0);
            let mut sol = Solution {
                field: vec![vec![false; w]; h],
            };
            let mut fill: Vec<_> = field
                .field
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|c| (c == &State::Empty).then_some(false))
                        .collect::<Vec<_>>()
                })
                .collect();
            let mut found = None;

            Self::dfs(field, 0, &mut sol, &mut fill, &mut found);

            found
        }
    }

    #[cfg(test)]
    mod test_backtrack {
        use crate::{
            field::{Field, Solution, State},
            solver::{MISMATCH_AKARI, OVERLAP_AKARI, Solver, UNLIT_CELL, solvers::Naive},
        };

        #[test]
        fn test_check() {
            // 成功
            let field = Field::from_str(3, 3, "2.1 ... ..0").unwrap();
            let sol = Solution {
                field: vec![
                    vec![false, true, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
            };
            assert_eq!(Naive::check(&field, &sol), Ok(()));

            // あかりの数の不一致
            let field = Field::from_str(3, 3, "2.2 ... ..0").unwrap();
            let sol = Solution {
                field: vec![
                    vec![false, true, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
            };
            assert_eq!(Naive::check(&field, &sol), Err(MISMATCH_AKARI));

            // あかりの重複
            let field = Field {
                h: 3,
                w: 3,
                field: vec![
                    vec![State::Adj2, State::Empty, State::Adj2],
                    vec![State::Empty, State::Empty, State::Empty],
                    vec![State::Empty, State::Empty, State::Empty],
                ],
            };
            let sol = Solution {
                field: vec![
                    vec![false, true, false],
                    vec![true, false, true],
                    vec![false, false, false],
                ],
            };
            assert_eq!(Naive::check(&field, &sol), Err(OVERLAP_AKARI));

            // 照らされていないマスが存在
            let field = Field::from_str(3, 3, "2.1 ... ...").unwrap();
            let sol = Solution {
                field: vec![
                    vec![false, true, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
            };
            assert_eq!(Naive::check(&field, &sol), Err(UNLIT_CELL));
        }

        #[test]
        fn test_solve() {
            let field = Field::from_str(1, 3, ".2.").unwrap();
            let answer = Solution {
                field: vec![vec![true, false, true]],
            };
            assert_eq!(Naive.solve(&field), Some(answer));

            let field = Field::from_str(3, 3, "2.1 ... ..0").unwrap();
            let answer = Solution {
                field: vec![
                    vec![false, true, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
            };
            assert_eq!(Naive.solve(&field), Some(answer));
        }
    }
}
