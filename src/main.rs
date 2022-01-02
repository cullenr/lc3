mod lc3;

pub use lc3::*;

fn main() {
    let vm = lc3::Vm::new();
    vm.run_program([1, 2, 3]);
    println!("OK");
}
