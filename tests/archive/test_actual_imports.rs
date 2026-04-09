use wasmtime_wasi::sync;
use wasmtime_wasi::cli;
use wasmtime_wasi::p2;

fn main() {
    println!("Testing imports");
    println!("sync: {:?}", std::any::type_name::<sync::WasiCtxBuilder>());
    println!("cli module exists");
    println!("p2 module exists");
}
