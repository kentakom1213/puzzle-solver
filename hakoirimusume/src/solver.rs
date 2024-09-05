//! ソルバ

use std::collections::VecDeque;

use fxhash::FxHashMap;

use crate::state::State;

/// ソルバ
pub struct Solver {
    /// 状態
    pub state: Vec<State>,
    /// { 状態 : (id, 親ノードのid) }
    pub memo: FxHashMap<State, (usize, usize)>,
}

impl Solver {
    /// ソルバを初期化する
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            memo: FxHashMap::from_iter([(State::get_initial_state(), (0, usize::MAX))]),
        }
    }

    /// 解を探索する
    pub fn solve(&mut self) -> State {
        let mut queue = VecDeque::new();
        let initial_state = State::get_initial_state();
        queue.push_back((initial_state.clone(), 0));
        self.state.push(initial_state);

        while let Some((state, pid)) = queue.pop_front() {
            for next_state in state.next_states() {
                if self.memo.contains_key(&next_state) {
                    continue;
                }

                let id = self.state.len();
                self.state.push(next_state.clone());
                self.memo.insert(next_state.clone(), (id, pid));
                queue.push_back((next_state, id));

                // 解が見つかったら終了
                if state.is_goal() {
                    return state.clone();
                }
            }
        }

        panic!("解が見つかりませんでした");
    }

    /// 解を復元する
    pub fn restore(&self, state: &State) -> Vec<State> {
        let mut states = Vec::new();
        let mut id = self.memo[state].0;

        while id != usize::MAX {
            states.push(self.state[id].clone());
            id = self.memo[&self.state[id]].1;
        }

        states.reverse();
        states
    }
}
