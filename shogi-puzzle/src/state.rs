//! 盤面を保存する

use colored::Colorize;

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
#[derive(PartialEq, Eq, Hash)]
pub struct Field([u32; 7]);

impl Field {
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
    pub fn get_initial_state() -> Field {
        Field([
            0b_0000_0000_0000_0110_1001,
            0b_0000_0000_0001_0001_0000,
            0b_0000_0000_1000_1000_0000,
            0b_0001_0001_0000_0000_0000,
            0b_1000_1000_0000_0000_0000,
            0b_0000_0000_0110_0000_0000,
            0b_0110_0110_0000_0000_0000,
        ])
    }

    /// 整形して表示する
    pub fn prerry_print(&self) {
        let mut field: [[u8; 4]; 5] = [[0; 4]; 5];

        for d in 0..7 {
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
                    1 => "▓▓".green().to_string(),
                    2 => "▓▓".cyan().to_string(),
                    3 => "▓▓".blue().to_string(),
                    4 => "▓▓".white().to_string(),
                    5 => "▓▓".purple().to_string(),
                    6 => "▓▓".yellow().to_string(),
                    7 => "▓▓".red().to_string(),
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
}
