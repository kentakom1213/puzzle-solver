use hakoirimusume::solver::Solver;

fn main() {
    let mut solver = Solver::new();

    let ans = solver.solve();
    let path = solver.restore(&ans);

    for (i, state) in path.iter().enumerate() {
        println!("{}手目", i);
        state.prerry_print();
    }
}
