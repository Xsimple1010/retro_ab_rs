extern crate libloading;

use libloading::Library;
use libloading::Symbol;
use libloading::os::unix::Symbol as SymbolRaw;

type RetroInit = unsafe extern "C" fn ();
type RetroApiVersion = unsafe extern "C" fn() -> i8;

pub struct CoreItl {
    handle: Library,
    retro_init: SymbolRaw<RetroInit>,
    retro_api_version: SymbolRaw<RetroApiVersion>,
}

impl  CoreItl {
    pub fn api_version (&self) -> i8 {
        unsafe {
            return (self.retro_api_version)();
        }
    }

    pub fn init(&self) {
        unsafe {
            (self.retro_init)()
        }
    }
}

fn load_symbol<T>(lib: &Library, symbol:&[u8]) -> SymbolRaw<T>{
    unsafe {
        let fun: Symbol<T> = lib.get(symbol).unwrap();
        fun.into_raw()
    }
}

pub fn load(path:&String) -> CoreItl  {
    unsafe {
        let lib: Library = libloading::Library::new(path).expect("fail to load core");
        
        let retro_init:SymbolRaw<RetroInit> = load_symbol::<RetroInit>(&lib, b"retro_init");
        let retro_api_version: SymbolRaw<RetroApiVersion> = load_symbol::<RetroApiVersion>(&lib, b"retro_api_version");

        let core_itl  = CoreItl {
            handle:lib,
            retro_init: retro_init,
            retro_api_version: retro_api_version,
        };

        core_itl
    }
}