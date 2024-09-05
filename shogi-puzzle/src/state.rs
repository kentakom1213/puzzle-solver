//! 盤面を保存する

use colored::Colorize;

/// 盤面の列数
const COLS: usize = 4;

/// ピースの数
const PIECES: usize = 10;

/// 盤面の左端
const LEFT_EDGE: u32 = 0b_0001_0001_0001_0001_0001;

/// 盤面の右端
const RIGHT_EDGE: u32 = 0b_1000_1000_1000_1000_1000;

/// 盤面の上端
const TOP_EDGE: u32 = 0b_0000_0000_0000_0000_1111;

/// 盤面の下端
const BOTTOM_EDGE: u32 = 0b_1111_0000_0000_0000_0000;

/// 盤面の状態を保存する
///
/// - `Field[0]` : 1x1の駒の位置
/// - `Field[1]` : 2x1 (縦長) の駒の位置 (1)
/// - `Field[2]` : 2x1 (縦長) の駒の位置 (2)
/// - `Field[3]` : 2x1 (縦長) の駒の位置 (3)
/// - `Field[4]` : 2x1 (縦長) の駒の位置 (4)
/// - `Field[5]` : 1x2 (横長) の駒の位置
/// - `Field[6]` : 2x2の駒の位置
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
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Field(pub [u32; 10]);

impl Field {
    /// 初期盤面を生成する
    ///
    /// ```text
    /// 0 _ _ 1
    /// 4 2 3 5
    /// 4 8 8 5
    /// 6 9 9 7
    /// 6 9 9 7
    /// ```
    #[inline]
    pub fn get_initial_state() -> Self {
        Self([
            0b_0000_0000_0000_0000_0001,
            0b_0000_0000_0000_0000_1000,
            0b_0000_0000_0000_0010_0000,
            0b_0000_0000_0000_0100_0000,
            0b_0000_0000_0001_0001_0000,
            0b_0000_0000_1000_1000_0000,
            0b_0001_0001_0000_0000_0000,
            0b_1000_1000_0000_0000_0000,
            0b_0000_0000_0110_0000_0000,
            0b_0110_0110_0000_0000_0000,
        ])
    }

    /// 遷移可能な盤面を列挙する
    pub fn next_states<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
        (0..PIECES)
            .flat_map(|i| {
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
            })
            .filter(|state| {
                // 他の駒と重なっていないかチェック
                (0..PIECES)
                    .fold(0, |mask, i| mask | state.0[i])
                    .count_ones()
                    == 18
            })
    }

    /// 終了状態かどうかを判定する
    pub fn is_goal(&self) -> bool {
        self.0[9] == 0b_0000_0000_0000_0110_0110
    }

    /// 整形して表示する
    pub fn prerry_print(&self) {
        let mut field: [[u8; 4]; 5] = [[0; 4]; 5];

        for d in 0..PIECES {
            for i in 0..20 {
                let (r, c) = (i / 4, i % 4);
                if self.0[d] >> i & 1 == 1 {
                    field[r][c] = d as u8 + 1;
                }
            }
        }

        let mut res = "┏━━────━━┓\n".to_string();

        for i in 0..5 {
            res += "┃";
            for j in 0..4 {
                res += &match field[i][j] {
                    0 => "  ".to_string(),
                    1..=4 => "▓▓".green().to_string(),
                    5 => "▓▓".cyan().to_string(),
                    6 => "▓▓".blue().to_string(),
                    7 => "▓▓".white().to_string(),
                    8 => "▓▓".purple().to_string(),
                    9 => "▓▓".yellow().to_string(),
                    10 => "▓▓".red().to_string(),
                    _ => unreachable!(),
                };
            }
            res += "┃\n";
        }

        res += "┗━━━━━━━━┛\n";

        print!("{}", res);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let field = Field::get_initial_state();
        field.prerry_print();
    }

    #[test]
    fn test_next_states() {
        let field = Field::get_initial_state();
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
        let field = Field::get_initial_state();
        field.prerry_print();
        assert!(!field.is_goal());

        let field = Field([
            0b_1000_0000_0000_0000_0000,
            0b_0001_0000_0000_0000_0000,
            0b_0000_0100_0000_0000_0000,
            0b_0000_0010_0000_0000_0000,
            0b_0000_0000_0000_0001_0001,
            0b_0000_0000_0000_1000_1000,
            0b_0000_0001_0001_0000_0000,
            0b_0000_1000_1000_0000_0000,
            0b_0110_0000_0000_0000_0000,
            0b_0000_0000_0000_0110_0110,
        ]);
        field.prerry_print();
        assert!(field.is_goal());
    }
}
