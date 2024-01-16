// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let x = call_me(3);
    println!("{x}");
}

fn call_me(num: u8) -> u8 {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }

    num
}
