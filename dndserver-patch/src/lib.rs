use retour::static_detour;

mod utils;

#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || unsafe { main_thread(hinst) });
    }
    
    if reason == 0 {
        utils::free_console();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    1
}

type FnSomeFunction = unsafe extern "fastcall" fn(
    a1: *const i64) -> i64;
static_detour! {
    static SomeFunction: unsafe extern "fastcall" fn(
        *const i64) -> i64;
}
fn hk_some_function(
    _a1: *const i64) -> i64 {
    
    1i64
}

type FnSomeFunction2 = unsafe extern "fastcall" fn(
    a1: i64) -> i64;
static_detour! {
    static SomeFunction2: unsafe extern "fastcall" fn(
        i64) -> i64;
}
fn hk_some_function2(
    _a1: i64) -> i64 {
    1i64
}

// 7FF61D537480 function add
// 7FF61D5376B6 terminate address
unsafe fn patch_multiclient() -> bool {
    let address = utils::get_base_address() + 0x21F7480;
    let fn_some_function: FnSomeFunction = std::mem::transmute(address as *const u64);
    SomeFunction
        .initialize(fn_some_function, hk_some_function)
        .unwrap()
        .enable()
        .unwrap();

    fn_some_function as u64 > 0
}

unsafe fn main_thread(_hinst: usize) {
    utils::alloc_console();
    if patch_multiclient() {
        println!("[-] dndserver-launcher: successfully injected patch\n")
    }
}