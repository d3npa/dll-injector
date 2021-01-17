// use detour::static_detour;
use winapi::shared::windef::{HWND};
use winapi::shared::minwindef::{HMODULE, DWORD, LPVOID};
use winapi::um::winnt::{LPCWSTR, DLL_PROCESS_ATTACH};

fn onload() {
    println!("Hello world");
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn DllMain(
    hinstDLL: HMODULE, fdwReason: DWORD, lpReserved: LPVOID) {
    match fdwReason {
        DLL_PROCESS_ATTACH => {
            onload();
        },
        _ => {},
    }
}