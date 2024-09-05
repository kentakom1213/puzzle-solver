//! ソルバ

use fxhash::FxHashMap;

use crate::state::Field;

/// ソルバ
pub struct Solver {
    /// { 状態 : (id, 親ノードのid) }
    states: FxHashMap<Field, (usize, usize)>,
}

impl Solver {
    /// ソルバを初期化する
    pub fn new() -> Self {
        Self {
            states: FxHashMap::from_iter([(Field::get_initial_state(), (0, usize::MAX))]),
        }
    }

    
}
