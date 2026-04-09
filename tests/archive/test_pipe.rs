use wasmtime_wasi::sync;

fn main() {
    println!("Testing pipe availability");
    let _pipe = sync::pipe::WritePipe::new_in_memory();
    println!("Pipe created successfully");
}
