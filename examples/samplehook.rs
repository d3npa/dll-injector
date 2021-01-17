use winapi::shared::minwindef::{HMODULE, DWORD, LPVOID};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

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