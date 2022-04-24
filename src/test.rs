//! Basic testing framework for `nstd`.
use std::time::Instant;

/// Basic testing framework for `nstd`.
#[allow(dead_code)]
pub fn run_test<F: Fn()>(test: F) {
    let mut scores = Vec::new();
    let mut score = 0_u128;
    let mut secs_passed = 0;
    let mut now = Instant::now();
    loop {
        if now.elapsed().as_secs_f32() > 1.0 {
            scores.push(score);
            secs_passed += 1;
            println!("[{secs_passed}]: {score}");
            if secs_passed >= 60 {
                break;
            }
            score = 0;
            now = Instant::now();
        } else {
            score += 1;
            test();
        }
    }
    let mut sum = 0_u128;
    for score in scores {
        sum += score;
    }
    let avg = sum / 60;
    println!("Average: {avg}");
}
