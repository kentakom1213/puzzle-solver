use shogi_puzzle::solver::Solver;

fn main() {
    let mut solver = Solver::new();

    solver.solve();

    println!("{:?}", solver.states.len());
}
