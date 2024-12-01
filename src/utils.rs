use std::time;


/// Benchmarking function, which runs the ``func``, ``n_times``, and
/// prints the mean execution time in microseconds.
pub fn benchmark(func: fn () -> (), n_times: usize) {
    let mut time_s = time::SystemTime::now();
    let mut sum = 0;
    for _ in 0..n_times {
        func();
        sum += time_s.elapsed().unwrap().as_micros();
        time_s = time::SystemTime::now();
    }

    println!("{}", sum as f64 / n_times as f64);
}
