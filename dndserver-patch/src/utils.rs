use windows::{
    Win32::{
        System::{
            Console::{AllocConsole, FreeConsole}, LibraryLoader::GetModuleHandleW,            
        },
    }, core::PCWSTR,
    
};

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