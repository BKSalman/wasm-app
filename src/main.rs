use wasm_app::{MyState, Something};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi::{ResourceTable, WasiCtxBuilder};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = std::env::args().skip(1);
    let Some(wasm_file) = args.next() else {
        eprintln!("ERROR: Path to wasm file was not provided");
        return Err(());
    };

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = match Engine::new(&config) {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("ERROR: Failed to start wasmtime engine: {e}");
            return Err(());
        }
    };
    let component = match Component::from_file(&engine, wasm_file.clone()) {
        Ok(comp) => comp,
        Err(e) => {
            eprintln!("ERROR: Failed to compile component {wasm_file}: {e}");
            return Err(());
        }
    };

    let mut linker = Linker::new(&engine);
    if let Err(e) = Something::add_to_linker(&mut linker, |s: &mut MyState| s) {
        eprintln!("ERROR: Failed to link component imports: {e}");
        return Err(());
    }

    let wasi_ctx = WasiCtxBuilder::new()
        .allow_ip_name_lookup(false)
        .allow_tcp(false)
        .allow_udp(false)
        .build();

    let mut store = Store::new(
        &engine,
        MyState {
            ctx: wasi_ctx,
            table: ResourceTable::new(),
        },
    );

    let (something, _) = Something::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    something.call_start(store).await.unwrap();

    Ok(())
}
