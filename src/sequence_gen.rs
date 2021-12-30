pub fn fibonacci_sequence_generator (iterations: i8) {
    let mut iterations_completed: i8 = 0;
    let mut prev_num: u128 = 0;
    let mut next_num: u128 = 1;
    while iterations_completed <= iterations {
        if iterations_completed == 0 {
            println!("{}", prev_num);
            println!("{}", next_num);
        }
        println!("{}", prev_num + next_num);
        next_num = prev_num + next_num;
        prev_num = next_num - prev_num;
        iterations_completed += 1;
    }
}