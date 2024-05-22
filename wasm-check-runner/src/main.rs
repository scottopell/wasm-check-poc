use wasmtime::component::ResourceTable;
use wasmtime::Store;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

// references:
// https://docs.rs/wasmtime-wasi/latest/wasmtime_wasi/fn.add_to_linker_sync.html
fn main() -> wasmtime::Result<()> {
    // Instantiate the engine and store
    let engine = wasmtime::Engine::default();

    // Configure the linker
    let mut linker = wasmtime::component::Linker::<MyState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let mut wasi_builder = WasiCtxBuilder::new();

    let mut store = Store::new(
        &engine,
        MyState {
            ctx: wasi_builder.build(),
            table: ResourceTable::new(),
        },
    );

    // Load the component from disk
    let bytes = std::fs::read("../smoke-a/smoke_a_component.wasm")?;
    let component = wasmtime::component::Component::new(&engine, bytes)?;

    type Params = (String, f64, Vec<String>);
    linker
        .root()
        .func_wrap("reportmetric", |_store, params: Params| {
            let (metricname, value, tags) = params;
            // probably put an mpsc::sender on the Store
            println!(
                "reportmetric called: {:?}, {:?}, {:?}",
                metricname, value, tags
            );
            Ok(())
        })?;

    // Instantiate the component
    let instance = linker.instantiate(&mut store, &component)?;

    // Call the `run` function
    let func = instance
        .get_func(&mut store, "run")
        .expect("run export not found");
    let mut result = [];
    func.call(&mut store, &[], &mut result)?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Result of reportmetric: {:?}", result);

    Ok(())
}
