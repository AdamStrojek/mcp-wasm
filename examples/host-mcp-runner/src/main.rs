use wasmtime::component::*;
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

bindgen!("server" in "../../wit");

fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();

    let wasi = WasiCtxBuilder::new().inherit_stderr().build();
    let state = ComponentRunStates {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);

    let mut linker: Linker<ComponentRunStates> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let component = Component::from_file(&engine, "../guest-rust/target/wasm32-wasip2/debug/mcp_wasm.wasm")?;
    let bindings = Server::instantiate(&mut store, &component, &linker)?;

    let version = VersionInfo {
        name: "Host MCP Runner".to_string(),
        version: "0.1.0".to_string(),
    };

    let (srv_ver, cap) = bindings.call_initialize(&mut store, &version, &[])?;

    println!("HOST: Module Version: {:?}", srv_ver);
    println!("HOST: Received capabilities: {:?}", cap);

    // bindings.call_notify(&mut store, "initialized", None)?;

    let res = bindings.mcp_wasm_server_resources().call_list(&mut store)?;
    println!("HOST: List all resources: {:?}", res);

    let res = bindings.mcp_wasm_server_prompts().call_get_list(&mut store)?;

    println!("HOST: Get List of prompts: {:?}", &res);

    bindings.call_notify(&mut store, "Hello", None)?;

    Ok(())
}

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}
