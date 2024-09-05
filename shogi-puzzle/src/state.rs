//! 盤面を保存する

use colored::Colorize;

/// 盤面の列数
const COLS: usize = 4;

/// 盤面の行数
const ROWS: usize = 5;

/// 盤面の左端
const LEFT_EDGE: u32 = 0b_0001_0001_0001_0001_0001;

/// 盤面の右端
const RIGHT_EDGE: u32 = 0b_1000_1000_1000_1000_1000;

/// 盤面の上端
const TOP_EDGE: u32 = 0b_0000_0000_0000_0000_1111;

/// 盤面の下端
const BOTTOM_EDGE: u32 = 0b_1111_0000_0000_0000_0000;

/// 1x1のピースの番号
const P1X1: usize = 0;

/// 2x1のピースの番号
const P2X1: usize = 1;

/// 1x2のピースの番号
const P1X2: usize = 2;

/// 2x2のピースの番号
const P2X2: usize = 3;

/// 盤面の状態を保存する
///
/// - `State[0]` : 1x1の駒の位置（4つ分）
/// - `State[1]` : 2x1 (縦長) の駒の位置（4つ分）
/// - `State[2]` : 1x2 (横長) の駒の位置
/// - `State[3]` : 2x2の駒の位置
///
/// インデックスは以下の通り（各値のbitに対応）
///
/// ```text
///  0  1  2  3
///  4  5  6  7
///  8  9 10 11
/// 12 13 14 15
/// 16 17 18 19
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State(pub [u32; 4]);

impl State {
    /// 初期盤面を生成する
    ///
    /// ```text
    /// 0 _ _ 0
    /// 1 0 0 2
    /// 1 5 5 2
    /// 3 6 6 4
    /// 3 6 6 4
    /// ```
    #[inline]
    pub fn get_initial_state() -> Self {
        Self([
            0b_0000_0000_0000_0110_1001,
            0b_1001_1001_1001_1001_0000,
            0b_0000_0000_0110_0000_0000,
            0b_0110_0110_0000_0000_0000,
        ])
    }

    /// 遷移可能な盤面を列挙する
    pub fn next_states<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
        // 1x1の駒
        (0..20)
            .filter(move |&i| self.0[P1X1] >> i & 1 == 1)
            .flat_map(|i| {
                let mut next_states = Vec::with_capacity(4);

                // 右に移動
                if RIGHT_EDGE >> i & 1 == 0 {
                    let mut next = self.clone();
                    next.0[P1X1] ^= 1 << i;
                    next.0[P1X1] |= 1 << (i + 1);
                    next_states.push(next);
                }

                // 上に移動
                if TOP_EDGE >> i & 1 == 0 {
                    let mut next = self.clone();
                    next.0[P1X1] ^= 1 << i;
                    next.0[P1X1] |= 1 << (i - COLS);
                    next_states.push(next);
                }

                // 左に移動
                if LEFT_EDGE >> i & 1 == 0 {
                    let mut next = self.clone();
                    next.0[P1X1] ^= 1 << i;
                    next.0[P1X1] |= 1 << (i - 1);
                    next_states.push(next);
                }

                // 下に移動
                if BOTTOM_EDGE >> i & 1 == 0 {
                    let mut next = self.clone();
                    next.0[P1X1] ^= 1 << i;
                    next.0[P1X1] |= 1 << (i + COLS);
                    next_states.push(next);
                }

                next_states
            })
            // 2x1の駒
            .chain({
                // 4つの駒に分解
                let peaces = Self::decompose_2x1(self.0[P2X1]);

                (0..4).flat_map(move |j| {
                    let mut next_states = Vec::with_capacity(4);

                    // 右に移動
                    if peaces[j] & RIGHT_EDGE == 0 {
                        let mut next = self.clone();
                        next.0[P2X1] ^= peaces[j];
                        next.0[P2X1] ^= peaces[j] << 1;
                        next_states.push(next);
                    }

                    // 上に移動
                    if peaces[j] & TOP_EDGE == 0 {
                        let mut next = self.clone();
                        next.0[P2X1] ^= peaces[j];
                        next.0[P2X1] ^= peaces[j] >> COLS;
                        next_states.push(next);
                    }

                    // 左に移動
                    if peaces[j] & LEFT_EDGE == 0 {
                        let mut next = self.clone();
                        next.0[P2X1] ^= peaces[j];
                        next.0[P2X1] ^= peaces[j] >> 1;
                        next_states.push(next);
                    }

                    // 下に移動
                    if peaces[j] & BOTTOM_EDGE == 0 {
                        let mut next = self.clone();
                        next.0[P2X1] ^= peaces[j];
                        next.0[P2X1] ^= peaces[j] << COLS;
                        next_states.push(next);
                    }

                    next_states
                })
            })
            // 他の駒
            .chain((2..4).flat_map(|i| {
                let mut next_states = Vec::with_capacity(4);

                // 右に移動
                if self.0[i] & RIGHT_EDGE == 0 {
                    let mut next = self.clone();
                    next.0[i] <<= 1;
                    next_states.push(next);
                }

                // 上に移動
                if self.0[i] & TOP_EDGE == 0 {
                    let mut next = self.clone();
                    next.0[i] >>= COLS;
                    next_states.push(next);
                }

                // 左に移動
                if self.0[i] & LEFT_EDGE == 0 {
                    let mut next = self.clone();
                    next.0[i] >>= 1;
                    next_states.push(next);
                }

                // 下に移動
                if self.0[i] & BOTTOM_EDGE == 0 {
                    let mut next = self.clone();
                    next.0[i] <<= COLS;
                    next_states.push(next);
                }

                next_states
            }))
            .filter(|state| {
                // 他の駒と重なっていないかチェック
                (0..4).fold(0, |mask, i| mask | state.0[i]).count_ones() == 18
            })
    }

    /// 終了状態かどうかを判定する
    pub fn is_goal(&self) -> bool {
        self.0[P2X2] == 0b_0000_0000_0000_0110_0110
    }

    /// 整形して表示する
    pub fn prerry_print(&self) {
        let mut field: [[u8; 4]; 5] = [[0; 4]; 5];
        let &Self([p1x1, p2x1, p1x2, p2x2]) = self;

        // 4つの駒に分解
        let [p2x1_1, p2x1_2, p2x1_3, p2x1_4] = Self::decompose_2x1(p2x1);

        for (i, x) in [p1x1, p2x1_1, p2x1_2, p2x1_3, p2x1_4, p1x2, p2x2]
            .into_iter()
            .enumerate()
        {
            for j in 0..ROWS * COLS {
                let (r, c) = (j / COLS, j % COLS);
                if x >> j & 1 == 1 {
                    field[r][c] = i as u8 + 1;
                }
            }
        }

        let mut res = "┏━━────━━┓\n".to_string();

        for i in 0..ROWS {
            res += "┃";
            for j in 0..COLS {
                res += &match field[i][j] {
                    0 => "  ".to_string(),
                    1 => "██".green().to_string(),
                    2 => "██".cyan().to_string(),
                    3 => "██".blue().to_string(),
                    4 => "██".white().to_string(),
                    5 => "██".purple().to_string(),
                    6 => "██".yellow().to_string(),
                    7 => "██".red().to_string(),
                    _ => unreachable!(),
                };
            }
            res += "┃\n";
        }

        res += "┗━━━━━━━━┛\n";

        print!("{}", res);
    }

    /// 1つの変数にまとめられている2x1の駒を分解する
    fn decompose_2x1(mut peace2x1: u32) -> [u32; 4] {
        let mut idx = 0;
        let mut peaces = [0; 4];

        for j in 0..16 {
            if peace2x1 >> j & 1 == 1 {
                peaces[idx] |= 1 << j;
                peaces[idx] |= 1 << (j + COLS);
                peace2x1 ^= 1 << j;
                peace2x1 ^= 1 << (j + COLS);
                idx += 1;
            }
        }

        assert_eq!(idx, 4);
        peaces
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let field = State::get_initial_state();
        field.prerry_print();
    }

    #[test]
    fn test_next_states() {
        let field = State::get_initial_state();
        println!("Initial State");
        field.prerry_print();

        let next_states = field.next_states().collect::<Vec<_>>();

        println!("Next States");
        for state in next_states {
            state.prerry_print();
        }

        let field2 = field.next_states().nth(2).unwrap();
        println!("2nd State");
        field2.prerry_print();

        println!("Next States");
        for state in field2.next_states() {
            state.prerry_print();
        }

        let field3 = field2.next_states().nth(3).unwrap();
        println!("3rd State");
        field3.prerry_print();

        println!("Next States");
        for state in field3.next_states() {
            state.prerry_print();
        }

        let field4 = field3.next_states().nth(2).unwrap();
        println!("4th State");
        field4.prerry_print();

        println!("Next States");
        for state in field4.next_states() {
            state.prerry_print();
        }
    }

    #[test]
    fn test_is_goal() {
        let field = State::get_initial_state();
        field.prerry_print();
        assert!(!field.is_goal());

        let field = State([
            0b_1001_0110_0000_0000_0000,
            0b_1001_1001_1001_1001_0000,
            0b_0110_0000_0000_0000_0000,
            0b_0000_0000_0000_0110_0110,
        ]);
        field.prerry_print();
        assert!(field.is_goal());
    }
}
