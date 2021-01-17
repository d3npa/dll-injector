use std::mem;
use std::ffi::CString;
use std::ptr::null_mut;

use winapi::shared::minwindef::{LPVOID, DWORD, FALSE};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::processthreadsapi::{OpenProcess, CreateRemoteThread};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::winnt::{
    PROCESS_CREATE_THREAD,
    PROCESS_VM_OPERATION,
    PROCESS_VM_WRITE,
    MEM_RESERVE,
    MEM_COMMIT,
    PAGE_READWRITE,
};

// TODO: 実装
// pub fn find_pid(_target: &str) -> Option<u32> {
//     // PROCESSENTRY32を取得しth32ProcessIDを返す
//     None
// }

fn get_fn_addr<'a>(mod_name: &str, fn_name: &str) -> Result<u64, &'a str> {
    let mod_str = CString::new(mod_name).unwrap();
    let fn_str = CString::new(fn_name).unwrap();

    let mod_handle = unsafe {
        GetModuleHandleA(mod_str.as_ptr())
    };

    if mod_handle == null_mut() {
        eprintln!("{:?} のハンドラの取得ができませんでした", mod_str);
        return Err("GetModuleHandleA");
    }

    let fn_addr = unsafe {
        GetProcAddress(mod_handle, fn_str.as_ptr())
    };

    if fn_addr == null_mut() {
        eprintln!("{:?} のアドレスを取得できませんでした", fn_addr);
        return Err("GetProcAddress");
    }

    println!("LoadLibraryA のアドレスを解決しました: {:?}", fn_addr);

    Ok(fn_addr as u64)
}

pub fn inject_dll(target_pid: u32, dll_path: &str) -> Result<(), &str> {
    // RustはLoadLibraryAのアドレスを知らないからKernel32から取得する
    let fn_lla_addr = match get_fn_addr("Kernel32.dll", "LoadLibraryA") {
        Ok(addr) => addr,
        Err(e) => return Err(e),
    };

    let dll_path_str = CString::new(dll_path).unwrap();
    let dll_path_size = dll_path_str.as_bytes_with_nul().len();

    // DLLパスを対象プロセスのメモリに書き込む
    let proc = unsafe {
        OpenProcess(
            PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, 
            FALSE, 
            target_pid)
    };

    if proc == null_mut() {
        eprintln!("PID「{}」を持ったプロセスにアタッチできませんでした", target_pid);
        return Err("OpenProcess");
    }

    let va_path = unsafe {
        VirtualAllocEx(
            proc, 
            null_mut(), 
            dll_path_size, 
            MEM_RESERVE | MEM_COMMIT, 
            PAGE_READWRITE)
    };

    if va_path == null_mut() {
        eprintln!(
            "対象プロセスのアドレス空間でメモリ領域を確保できませんでした");
        return Err("VirtualAllocEx");
    }

    let write_success = unsafe {
        WriteProcessMemory(
            proc, 
            va_path, 
            dll_path_str.as_ptr() as LPVOID,
            dll_path_size,
            null_mut())
    };

    if write_success == FALSE {
        eprintln!("対象プロセスのメモリに書き込むことができませんでした");
        return Err("WriteProcessMemory");
    }

    unsafe {
        type ThreadStartRoutine = unsafe extern "system" fn(LPVOID) -> DWORD;
        let start_routine: ThreadStartRoutine = mem::transmute(fn_lla_addr);
        CreateRemoteThread(
            proc, 
            null_mut(), 
            0,
            Some(start_routine),
            va_path,
            0,
            null_mut());
    }

    Ok(())
}

