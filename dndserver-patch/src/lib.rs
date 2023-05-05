
use retour::static_detour;

mod utils;
mod sdk;
mod ue;

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

#[allow(unused_assignments)]
unsafe fn main_thread(_hinst: usize) {
    utils::alloc_console();

    if patch_multiclient() {
        println!("[-][dndserver-launcher] patched multiclient, waiting for objects\n")
    }

    ue::G_OBJECTS = Some(utils::get_g_objects());
    ue::G_NAMES = Some(utils::get_g_names());
    ue::G_WORLD = Some(utils::get_g_world());
    
    let world = &*ue::G_WORLD.unwrap();
    let g_objects = &*ue::G_OBJECTS.unwrap();

    while *world as u64 == 0x0 || (*g_objects).len() < 1 {
        std::thread::sleep(std::time::Duration::from_millis(100));        
    }

    println!("\n[-][dndserver-launcher] world: {:X}", *world as u64);
    let game = (*(*world)).owning_game_instance;
    println!("[-][dndserver-launcher] game: {:X}", game as u64);
    let players = &(*game).local_players;
    println!("[-][dndserver-launcher] localplayers count: {:X}", players.len());
    let localplayer = players.get(0);
    println!("[-][dndserver-launcher] localplayer: {:X}", localplayer as u64);
    let mut viewport = (*localplayer).viewport_client as *mut sdk::UGameViewportClient;
    println!("[-][dndserver-launcher] viewport: {:X}", viewport as u64);
    
    let mut console_class: *const ue::UClass = std::ptr::null_mut();
    let mut gameplay_statics: *const sdk::UGameplayStatics = std::ptr::null_mut();
    loop {
        console_class = g_objects.find_object("Class Engine.Console") as *const ue::UClass;
        gameplay_statics = g_objects.find_object("Class Engine.GameplayStatics") as *const sdk::UGameplayStatics;
        if !console_class.is_null() && !gameplay_statics.is_null() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let console = (*gameplay_statics).spawn_object(console_class, &(*viewport).script_viewport_client_.object_ as *const _ as *const ue::UObject);
    (*viewport).viewport_console = console as *const sdk::UConsole;

    println!("[-][dndserver-launcher] successfully initiated\n")
}