use std::io::Write;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

// Create a simple in-memory writer
struct MemoryWriter(Vec<u8>);

impl Write for MemoryWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing MemoryWriter as stdout");
    
    let writer = MemoryWriter(Vec::new());
    
    // Try to use it as stdout
    let wasi = WasiCtxBuilder::new()
        .stdout(writer)
        .build()?;
    
    println!("Success! WasiCtxBuilder accepts MemoryWriter");
    Ok(())
}
