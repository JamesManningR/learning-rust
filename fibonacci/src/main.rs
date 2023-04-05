//  0 ->    0
//  1 ->    1
//  2 ->    1
//  3 ->    2
//  4 ->    3
//  5 ->    5
//  6 ->    8
//  7 ->   13
//  8 ->   21
//  9 ->   34
// 10 ->   55
// 11 ->   89
// 12 ->  144
// 13 ->  233
// 14 ->  377
// 15 ->  610
// 16 ->  987
// 17 -> 1597
// 18 -> 2584
// 19 -> 4181
// 20 -> 6765

fn fibonacci_rec (n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fibonacci_rec(n-1) + fibonacci_rec(n-2)
    }
}

fn fibonacci_loop (n: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;
    let mut c: i64;

    for _i in 0..n {
        c = a + b;
        a = b;
        b = c;
    }

    a
}

fn bench_recur(iterations: i64) {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        fibonacci_rec(iterations);
    }

    let elapsed = now.elapsed();
    println!("Recur Elapsed: {:.2?}", elapsed);
}

fn bench_loop(iterations: i64) {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        fibonacci_loop(iterations);
    }

    let elapsed = now.elapsed();
    println!("Loop Elapsed: {:.2?}", elapsed);
}

fn main() {
    println!("Recursive Implementation ====================");

    for i in 0..21 {
        println!("{i} {}", fibonacci_rec(i))
    }

    println!("Loop Implementation ====================");

    for i in 0..21 {
        println!("{i} {}", fibonacci_loop(i))
    }

    // let test_iterations = 50;
    let test_iterations = 25;
    bench_recur(test_iterations);
    bench_loop(test_iterations);
}
