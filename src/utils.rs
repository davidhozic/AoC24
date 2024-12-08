use std::time;


/// Benchmarking function, which runs the ``func``, ``n_times``, and
/// prints the mean execution time in microseconds.
pub fn benchmark<T>(func: fn () -> T, n_times: usize) -> T {
    let mut time_s = time::SystemTime::now();
    let mut sum = 0;
    let mut r_val = None;
    for _ in 0..n_times {
        r_val = Some(func());
        sum += time_s.elapsed().unwrap().as_micros();
        time_s = time::SystemTime::now();
    }

    println!("{}", sum as f64 / n_times as f64);
    return r_val.unwrap();
}


/// Prints characters of a 2D map to the console
/// in a formatted way.
pub fn print_map(map: &Vec<Vec<char>>) {
    for l in map {
        for c in l {
            print!("{c}");
        }
        println!("");
    }
}
