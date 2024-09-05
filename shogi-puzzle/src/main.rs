use shogi_puzzle::{solver::Solver, state::State};

fn main() {
    let mut solver = Solver::new();

    let ans = solver.solve();
    let path = solver.restore(&ans);

    // 初期状態
    println!("初期状態");
    State::get_initial_state().prerry_print();

    for (i, state) in path.iter().enumerate() {
        println!("{}手目", i + 1);
        state.prerry_print();
    }
}
