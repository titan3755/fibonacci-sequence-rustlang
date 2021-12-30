mod sequence_gen;
fn main () {
    let iters = 100;
    println!("A fibonacci sequence of {} iterations is given below --->\n", iters);
    sequence_gen::fibonacci_sequence_generator(iters);
}