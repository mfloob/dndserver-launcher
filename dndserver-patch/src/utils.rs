use windows::{
    Win32::{
        System::{
            Console::{AllocConsole, FreeConsole}, LibraryLoader::GetModuleHandleW,            
        },
    }, core::PCWSTR,
    
};

use crate::{ue, sdk};

pub unsafe fn get_g_objects() -> *const ue::TUObjectArray {
    let base = get_base_address();
    std::mem::transmute(base + 0x7989C10)
}

pub unsafe fn get_g_names() -> *const ue::FNamePool {
    let base = get_base_address();
    std::mem::transmute(base + 0x78EA280)
}

pub unsafe fn get_g_world() -> *const *const sdk::UWorld {
    let base = get_base_address();
    std::mem::transmute(base + 0x7AF7D50)
}

pub fn get_base_address() -> u64 {
    unsafe {
        GetModuleHandleW(PCWSTR(std::ptr::null())).unwrap().0 as u64
    }
}

pub fn alloc_console() {
    unsafe {
        AllocConsole();
    }
}

pub fn free_console() {
    unsafe {
        FreeConsole();
    }
}