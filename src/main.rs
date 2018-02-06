mod memory;

fn main() {
    let memory = memory::new();
    println!("x = {}", memory.mem[0]);
    println!("x = {}", memory.get_gpr(1));
}
