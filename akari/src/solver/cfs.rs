//! 制約を優先的に探索

use itertools::Itertools;

use crate::{
    Field, Solution, Solver, State,
    solver::OVERLAP_AKARI,
    utility::{ADJ, GridUtility},
};

/// セルの一時的な状態
#[derive(Debug, Clone)]
pub enum Cell {
    /// あかりを置くことができるセル（照らされているか）
    Fillable(bool),
    /// あかりを置くことができないセル（照らされているか）
    Unfillable(bool),
    Nil,
}

impl Cell {
    fn as_mut<'a>(&'a mut self) -> Option<&'a mut bool> {
        match self {
            Self::Fillable(f) => Some(f),
            Self::Unfillable(f) => Some(f),
            Self::Nil => None,
        }
    }

    /// セルをあかりがおけない状態にする
    fn disable(&mut self) {
        match self {
            Self::Fillable(f) => *self = Self::Unfillable(*f),
            _ => {}
        }
    }

    /// セルにあかりを置くことができるかどうか
    fn can_put_akari(&self) -> bool {
        match self {
            Self::Fillable(false) => true,
            _ => false,
        }
    }
}

pub type TempFill = Vec<Vec<Cell>>;

/// constraint first search
///
/// 影響範囲が狭く強い制約を持つセル（数字セル）が「最も情報量の大きい変数」として優先される変数選択ヒューリスティック．
pub struct CFS;

impl CFS {
    fn rec(
        field: &Field,
        constraints: &[(usize, usize)],
        cons_pos: usize,
        cell_pos: usize,
        sol: Solution,
        fill: TempFill,
        found: &mut Option<Solution>,
    ) {
        // println!("{}", field.display_with_solution_and_state(&sol, &fill));

        let (h, w) = (field.h, field.w);

        if found.is_some() {
            return;
        }

        // 最後のセルに来た場合，終了
        if cell_pos == h * w {
            if Self::_check(field, &sol).is_ok() {
                *found = Some(sol);
            }
            return;
        }

        // 制約が残っている場合
        if let Some(&(r, c)) = constraints.get(cons_pos) {
            match field.field[r][c] {
                State::Adj0 => {
                    if let Some((sol, fill)) = Some((sol, fill))
                        .map(|(sol, mut fill)| {
                            if let Some((ar, ac)) = (r, c).right(h, w) {
                                fill[ar][ac].disable();
                            }
                            (sol, fill)
                        })
                        .map(|(sol, mut fill)| {
                            if let Some((ar, ac)) = (r, c).up(h, w) {
                                fill[ar][ac].disable();
                            }
                            (sol, fill)
                        })
                        .map(|(sol, mut fill)| {
                            if let Some((ar, ac)) = (r, c).left(h, w) {
                                fill[ar][ac].disable();
                            }
                            (sol, fill)
                        })
                        .map(|(sol, mut fill)| {
                            if let Some((ar, ac)) = (r, c).down(h, w) {
                                fill[ar][ac].disable();
                            }
                            (sol, fill)
                        })
                    {
                        Self::rec(field, constraints, cons_pos + 1, cell_pos, sol, fill, found);
                    }
                }
                State::Adj1 => {
                    // 1 方向へのあかりの置き方を 4 通り試す
                    for d in ADJ {
                        // 置かない方向
                        let nd: Vec<_> = ADJ.into_iter().filter(|&x| x != d).collect();

                        if let Some((sol, fill)) = (r, c)
                            .dir(h, w, d)
                            .and_then(|(ar, ac)| {
                                Self::put_akari(field, ar, ac, sol.clone(), fill.clone()).ok()
                            })
                            // 置けない場所を設定
                            .map(|(sol, mut fill)| {
                                if let Some((ar, ac)) = (r, c).dir(h, w, nd[0]) {
                                    fill[ar][ac].disable();
                                }
                                (sol, fill)
                            })
                            .map(|(sol, mut fill)| {
                                if let Some((ar, ac)) = (r, c).dir(h, w, nd[1]) {
                                    fill[ar][ac].disable();
                                }
                                (sol, fill)
                            })
                            .map(|(sol, mut fill)| {
                                if let Some((ar, ac)) = (r, c).dir(h, w, nd[2]) {
                                    fill[ar][ac].disable();
                                }
                                (sol, fill)
                            })
                        {
                            Self::rec(field, constraints, cons_pos + 1, cell_pos, sol, fill, found);
                        }
                    }
                }
                State::Adj2 => {
                    // 2 方向へのあかりの置き方を 6 通り試す
                    for d in ADJ.iter().combinations(2) {
                        // 置かない方向
                        let nd: Vec<_> = ADJ.into_iter().filter(|x| !d.contains(&x)).collect();

                        if let Some((sol, fill)) = Some((sol.clone(), fill.clone()))
                            .and_then(|(sol, fill)| {
                                (r, c).dir(h, w, *d[0]).and_then(|(ar, ac)| {
                                    Self::put_akari(field, ar, ac, sol, fill).ok()
                                })
                            })
                            .and_then(|(sol, fill)| {
                                (r, c).dir(h, w, *d[1]).and_then(|(ar, ac)| {
                                    Self::put_akari(field, ar, ac, sol, fill).ok()
                                })
                            })
                            // 置けない場所を設定
                            .map(|(sol, mut fill)| {
                                if let Some((ar, ac)) = (r, c).dir(h, w, nd[0]) {
                                    fill[ar][ac].disable();
                                }
                                (sol, fill)
                            })
                            .map(|(sol, mut fill)| {
                                if let Some((ar, ac)) = (r, c).dir(h, w, nd[1]) {
                                    fill[ar][ac].disable();
                                }
                                (sol, fill)
                            })
                        {
                            Self::rec(field, constraints, cons_pos + 1, cell_pos, sol, fill, found);
                        }
                    }
                }
                State::Adj3 => {
                    // 3 方向へのあかりの置き方を 4 通り試す
                    for d in ADJ.iter().combinations(3) {
                        // 置かない方向
                        let nd: Vec<_> = ADJ.into_iter().filter(|x| !d.contains(&x)).collect();

                        if let Some((sol, fill)) = Some((sol.clone(), fill.clone()))
                            .and_then(|(sol, fill)| {
                                (r, c).dir(h, w, *d[0]).and_then(|(ar, ac)| {
                                    Self::put_akari(field, ar, ac, sol, fill).ok()
                                })
                            })
                            .and_then(|(sol, fill)| {
                                (r, c).dir(h, w, *d[1]).and_then(|(ar, ac)| {
                                    Self::put_akari(field, ar, ac, sol, fill).ok()
                                })
                            })
                            .and_then(|(sol, fill)| {
                                (r, c).dir(h, w, *d[2]).and_then(|(ar, ac)| {
                                    Self::put_akari(field, ar, ac, sol, fill).ok()
                                })
                            })
                            // 置けない場所を設定
                            .map(|(sol, mut fill)| {
                                if let Some((ar, ac)) = (r, c).dir(h, w, nd[0]) {
                                    fill[ar][ac].disable();
                                }
                                (sol, fill)
                            })
                        {
                            Self::rec(field, constraints, cons_pos + 1, cell_pos, sol, fill, found);
                        }
                    }
                }
                State::Adj4 => {
                    // 全方向にあかりを置く
                    if let Some((sol, fill)) = Some((sol, fill))
                        .and_then(|(sol, fill)| {
                            (r, c)
                                .right(h, w)
                                .and_then(|(ar, ac)| Self::put_akari(field, ar, ac, sol, fill).ok())
                        })
                        .and_then(|(sol, fill)| {
                            (r, c)
                                .up(h, w)
                                .and_then(|(ar, ac)| Self::put_akari(field, ar, ac, sol, fill).ok())
                        })
                        .and_then(|(sol, fill)| {
                            (r, c)
                                .left(h, w)
                                .and_then(|(ar, ac)| Self::put_akari(field, ar, ac, sol, fill).ok())
                        })
                        .and_then(|(sol, fill)| {
                            (r, c)
                                .down(h, w)
                                .and_then(|(ar, ac)| Self::put_akari(field, ar, ac, sol, fill).ok())
                        })
                    {
                        Self::rec(field, constraints, cons_pos + 1, cell_pos, sol, fill, found);
                    }
                }
                _ => unreachable!(),
            }
            return;
        }

        // 制約が残っていない場合，愚直に埋めていく
        let (r, c) = (cell_pos / w, cell_pos % w);

        // あかりが設置できる場合
        if fill[r][c].can_put_akari() {
            // あかりを設置
            if let Ok((sol, fill)) = Self::put_akari(field, r, c, sol.clone(), fill.clone()) {
                Self::rec(field, constraints, cons_pos, cell_pos + 1, sol, fill, found);
            }
        }

        // あかりを設置しない
        Self::rec(field, constraints, cons_pos, cell_pos + 1, sol, fill, found);
    }

    /// field の (r, c) にあかりを配置する
    fn put_akari(
        field: &Field,
        r: usize,
        c: usize,
        mut sol: Solution,
        mut fill: TempFill,
    ) -> Result<(Solution, TempFill), &'static str> {
        // その場を塗れるか確認
        if let Cell::Fillable(fcell) = &mut fill[r][c] {
            // すでにおいてある場合，そのまま
            if sol.field[r][c] {
                return Ok((sol, fill));
            }
            *fcell = true;
        } else {
            return Err("Given cell is not fillable.");
        }
        // あかりを設置
        sol.field[r][c] = true;

        // 重複確認
        for dir in ADJ {
            // 特定方向に塗れるだけ塗る
            for (nr, nc) in (r, c).while_dir(field.h, field.w, dir) {
                // あかりが置かれていたら失敗
                if sol.field[nr][nc] {
                    return Err(OVERLAP_AKARI);
                }
                // ブロックに当たったら終了
                if let Some(cell) = fill[nr][nc].as_mut() {
                    *cell = true;
                } else {
                    break;
                }
            }
        }

        Ok((sol, fill))
    }
}

impl Solver for CFS {
    fn solve(&self, field: &Field) -> Option<Solution> {
        let h = field.field.len();
        let w = field.field.first().as_ref().map(|r| r.len()).unwrap_or(0);
        let sol = Solution {
            field: vec![vec![false; w]; h],
        };
        let constraints: Vec<_> = (0..h * w)
            .map(|i| (i / w, i % w))
            .filter(|&(r, c)| field.field[r][c].is_adj().is_some())
            .collect();
        let fill: Vec<_> = field
            .field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| {
                        if c.is_empty() {
                            Cell::Fillable(false)
                        } else {
                            Cell::Nil
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let mut found = None;

        Self::rec(field, &constraints, 0, 0, sol, fill, &mut found);

        found
    }
}

#[cfg(test)]
mod test_cfs {
    use crate::{
        field::{Field, Solution},
        solver::{Solver, cfs::CFS},
    };

    #[test]
    fn test_solve() {
        let field = Field::from_str(1, 3, ".2.").unwrap();
        let answer = Solution {
            field: vec![vec![true, false, true]],
        };
        assert_eq!(CFS.solve(&field), Some(answer));

        let field = Field::from_str(3, 3, "2.1 ... ..0").unwrap();
        let answer = Solution {
            field: vec![
                vec![false, true, false],
                vec![true, false, false],
                vec![false, false, false],
            ],
        };
        assert_eq!(CFS.solve(&field), Some(answer));
    }
}
