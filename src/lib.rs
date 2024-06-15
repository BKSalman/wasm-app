use wasmtime::component::*;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

bindgen!({
    async: true
});

#[async_trait::async_trait]
impl component::wasm_app::wow::Host for MyState {
    async fn add(&mut self, a: i32, b: i32) -> i32 {
        println!("add was called");
        a + b
    }
}

#[async_trait::async_trait]
impl wasi::logging::logging::Host for MyState {
    async fn log(
        &mut self,
        _level: wasi::logging::logging::Level,
        _context: String,
        message: String,
    ) {
        println!("{message}");
    }
}

pub struct MyState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
