//! ソルバ

use fxhash::FxHashMap;

use crate::state::Field;

/// ソルバ
pub struct Solver {
    /// { 状態 : (id, 親ノードのid) }
    pub states: FxHashMap<Field, (usize, usize)>,
}

impl Solver {
    /// ソルバを初期化する
    pub fn new() -> Self {
        Self {
            states: FxHashMap::from_iter([(Field::get_initial_state(), (0, usize::MAX))]),
        }
    }

    /// 解を探索する
    pub fn solve(&mut self) {
        let initial_state = Field::get_initial_state();
        self.dfs(initial_state, 0);
    }

    /// DFS
    fn dfs(&mut self, state: Field, id: usize) {
        println!("id: {}", id);
        state.prerry_print();

        for next_state in state.next_states() {
            if self.states.contains_key(&next_state) {
                continue;
            }

            let next_id = self.states.len();
            self.states.insert(next_state.clone(), (next_id, id));
            self.dfs(next_state, next_id);
        }
    }
}
