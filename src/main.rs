mod vm;
mod ops;

fn main() {
    let mut vm = vm::Vm::new();
    vm.run_program([1, 2, 3]);
    println!("OK");
}
