extern crate winapi;
use winapi::{ DWORD
            , HINSTANCE
            , LPVOID
            , BOOL
            };

#[allow(non_snake_case)]
//#[no_mangle] 
// FIXME: this worked in x64 toolchain for some reason...
#[export_name="?Hook@@YAXXZ"] // This is how MSVC mangles this name.
pub fn Hook() {
}

#[allow(non_snake_case)]
#[no_mangle] 
pub extern "stdcall" fn DllMain(_module: HINSTANCE, _reason: DWORD, _reserved: LPVOID) -> BOOL { 
    true as BOOL
} 

