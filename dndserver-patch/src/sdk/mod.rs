use crate::{ue, utils};

#[repr(C)]
pub struct UConsole {
    pub object_: ue::UObject,
    pad_28: [u8; 0x10],
    console_target_player: *const ULocalPlayer,
    default_texture_black: *const u64,
    default_texture_white: *const u64,
    pad_50: [u8; 0x18],
    history_buffer: ue::TArray<ue::FString>,
    pad_78: [u8; 0xb8]
}

#[repr(C)]
pub struct UBlueprintFunctionLibrary {
    pub object_: ue::UObject
}

#[repr(C)]
pub struct UGameplayStatics {
    pub blueprint_function_library_: UBlueprintFunctionLibrary
}

#[repr(C)]
pub struct UScriptViewportClient {
    pub object_: ue::UObject,
    pad_28: [u8; 0x10]
}

#[repr(C)]
pub struct UGameViewportClient {
    pub script_viewport_client_: UScriptViewportClient,
    pad_38: [u8; 0x8],
    pub viewport_console: *const UConsole,
    pad_debug_properties: [u8; 0x10],
    pad_58: [u8; 0x10],
    max_splitscreen_players: i32,
    pad_6c: [u8; 0xc],
    world: *const UWorld,
    game_instance: *const UGameInstance,
    pad_88: [u8; 0x318]
}

#[repr(C)]
pub struct UPlayer {
    object_: ue::UObject,
    pad_28: [u8; 0x8],
    player_controller: *const u64,
    current_net_speed: i32,
    configured_internet_speed: i32,
    configured_lan_speed: i32,
    pad_44: [u8; 0x4]
}

#[repr(C)]
pub struct ULocalPlayer {
    player_: UPlayer,
    pad_48: [u8; 0x30],
    pub viewport_client: *const UGameViewportClient,
    pad_80: [u8; 0x38],
    aspect_ratio_axis_constraint: [u8; 0x1],
    pad_b9: [u8; 0x7],
    pending_level_player_controller_class: *const u64,
    sent_split_join: [u8; 0x1],
    pad_c8_1: [u8; 0x1],
    pad_c9: [u8; 0x17],
    controller_id: i32,
    pad_e4: [u8; 0x1b4]
}

#[repr(C)]
pub struct UGameInstance {
    object_: ue::UObject,
    pad_28: [u8; 0x10],
    pub local_players: ue::TArray<*const ULocalPlayer>,
    online_session: *const u64,
    referenced_objects: ue::TArray<*const ue::UObject>,
    pad_60: [u8; 0x18],
    pad_on_pawn_controller_changed_delegates: [u8; 0x10],
    pad_88: [u8; 0x18],
    pad_on_input_device_connection_change: [u8; 0x10],
    pad_on_user_input_device_pairing_change: [u8; 0x10],
    pad_c0: [u8; 0x100]
}

#[repr(C)]
pub struct UWorld {
    object_: ue::UObject,
    //pad_28: [u8; 0x8], // I don't know why alignment is wrong, this should be uncommented
    persistent_level: *const u64,
    net_driver: *const u64,
    line_batcher: *const u64,
    persistent_line_batcher: *const u64,
    foreground_line_batcher: *const u64,
    network_manager: *const u64,
    u_physics_collision_handler: *const u64,
    extra_referenced_objects: ue::TArray<*const ue::UObject>,
    per_module_data_objects: ue::TArray<*const ue::UObject>,
    streaming_levels: ue::TArray<*const u64>,
    pad_streaming_levels_to_consider: [u8; 0x28],
    server_streaming_levels_visibility: *const u64,
    streaming_levels_prefix: ue::FString,
    pad_d8: [u8; 0x8],
    current_level_pending_visibility: *const u64,
    current_level_pending_invisibility: *const u64,
    demo_net_driver: *const u64,
    my_particle_event_manager: *const u64,
    default_physics_volume: *const u64,
    pad_108: [u8; 0x36],
    pad_13e_0: [u8; 0x1],
    are_constraints_dirty: [u8; 0x1],
    pad_13e_3: [u8; 0x1],
    pad_13f: [u8; 0x9],
    navigation_system: *const u64,
    authority_game_mode: *const u64,
    game_state: *const u64,
    ai_system: *const u64,
    avoidance_manager: *const u64,
    levels: ue::TArray<*const u64>,
    pad_level_collections: [u8; 0x10],
    pad_190: [u8; 0x28],
    pub owning_game_instance: *const UGameInstance,
    parameter_collection_instances: ue::TArray<*const u64>,
    canvas_for_rendering_to_target: *const u64,
    canvas_for_draw_material_to_render_target: *const u64,
    pad_1e0: [u8; 0x70],
    physics_field: *const u64,
    lwi_last_assigned_uid: u32,
    pad_25c: [u8; 0x4],
    pad_components_that_need_pre_end_of_frame_sync: [u8; 0x50],
    components_that_need_end_of_frame_update: ue::TArray<*const u64>,
    components_that_need_end_of_frame_update_on_game_thread: ue::TArray<*const u64>,
    pad_2d0: [u8; 0x3f8],
    world_composition: *const u64,
    content_bundle_manager: *const u64,
    pad_6d8: [u8; 0xa8],
    pad_psc_pool: [u8; 0x58],
    pad_7d8: [u8; 0xc0]
}

#[repr(C)]
pub struct SpawnObjectParams {
    class: *const ue::UClass,
    outer: *const ue::UObject,
    return_val: *const ue::UObject
}

impl UGameplayStatics {
    pub unsafe fn spawn_object(&self, class: *const ue::UClass, outer: *const ue::UObject) -> *const ue::UObject {
        let g_objects = utils::get_g_objects();
        let spawn_object = (*g_objects).find_object("Function Engine.GameplayStatics.SpawnObject");

        let mut params = SpawnObjectParams {
            class,
            outer,
            return_val: std::ptr::null_mut()
        };

        self.blueprint_function_library_.object_.process_event(spawn_object as *const usize, &mut params as *mut _ as * mut usize);
        params.return_val as *const ue::UObject
    }
}