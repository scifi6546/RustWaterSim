use std::time::Instant;
use water_sim::FiniteSolver;
fn main() {
    let (mut solver, barriers) = FiniteSolver::barrier();
    let bench_start = Instant::now();
    for _ in 0..100_000 {
        solver.time_step(&barriers);
    }
    println!("time elapsed: {}s", bench_start.elapsed().as_secs_f32());
}
