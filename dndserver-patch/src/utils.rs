use windows::{
    Win32::{
        System::{
            Console::{AllocConsole, FreeConsole}
        },
    },
};

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