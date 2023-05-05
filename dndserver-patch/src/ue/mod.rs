#![allow(dead_code)]
use crate::{sdk};

#[repr(C)]
pub struct FNameEntryHandle {
    pub block: u32,
    pub offset: u32
}

impl FNameEntryHandle {
    fn new(block: u32, offset: u32) -> Self {
        Self {
            block,
            offset,
        }
    }

    fn index_to_handle(index: u32) -> FNameEntryHandle {
        let block = index >> 16;
        let offset = index & 65535;
        FNameEntryHandle::new(block, offset)
    }
}

#[repr(C)]
union NameUnion {
    ansi_name: [u8; 1024],
    wide_name: [u16; 1024]
}

#[repr(C)]
pub struct FNameEntry {
    flags: u16,
    name: NameUnion,
}

impl FNameEntry {
    fn is_wide(&self) -> bool {
        (self.flags & 0x1) != 0x0
    }

    fn len(&self) -> u16 {
        (self.flags >> 6) & 0x3FF
    }

    pub unsafe fn string(&self) -> String {
        if self.is_wide() {
            return String::new();
        }
        let name_bytes = &self.name.ansi_name[..self.len() as usize];
        String::from_utf8(name_bytes.to_vec()).unwrap_or(String::new())
    }
}

#[repr(C)]
pub struct FNamePool {
    pub lock: [u8; 8],
    pub current_block: u32,
    pub current_byte_cursor: u32,
    pub blocks: [*const u8; 8192],
}

impl FNamePool {
    pub unsafe fn get_entry(&self, handle: FNameEntryHandle) -> *const FNameEntry {
        let block_ptr = self.blocks[handle.block as usize];
        let offset = block_ptr as u64 + (2 * handle.offset as u64);
        let entry = offset as *const FNameEntry;
        
        entry
    }
}

#[repr(C)]
pub struct FName {
    pub index: u32,
    pub number: u32
}

impl FName {
    pub unsafe fn get_name(&self) -> String {
        let g_names = G_NAMES.unwrap();
        let handle = FNameEntryHandle::index_to_handle(self.index); 
        let entry = (*g_names).get_entry(handle);

        let mut name = (*entry).string();
        if self.number > 0 {
            name.push_str(format!("_{}", self.number.to_string()).as_str());
        };

        if let Some(pos) = name.rfind('/') {
            name = name[(pos+1)..].to_string();
        };

        name
    }
}

#[repr(C)]
pub struct UField {
    pub object_: UObject,
    pub pad_28: [u8; 0x8],
}

#[repr(C)]
pub struct UStruct {
    pub field_: UField,
    pub pad_30: [u8; 0x10],
    pub super_struct: *const UStruct,
    pub pad_48: [u8; 0x68]
}

#[repr(C)]
pub struct UClass {
    pub struct_: UStruct,
    pub pad_b0: [u8; 0x180]
}

#[repr(C)]
pub struct TUObjectArray {
    objects: *const *const u8,
    pre_allocated_objects: *const u8,
    max_elements: u32,
    num_elements: u32,
    max_chunks: u32,
    num_chunks: u32
}

impl TUObjectArray {
    pub unsafe fn find_object(&self, name: &str) -> *const UObject {
        for i in 0..self.num_elements {
            let object = self.get_object_by_index(i);
            if object.is_null() {
                continue;
            }

            let obj_name = (*object).get_full_name();
            if obj_name == name {
                return object as *const UObject;
            }
        }

        std::ptr::null()
    }

    unsafe fn get_object_by_index(&self, index: u32) -> *const UObject {
        if index >= self.num_elements {
            return std::ptr::null();
        }

        let chunk_index = index / 65536;
        if chunk_index >= self.num_chunks {
            return std::ptr::null();
        }

        let chunk = *self.objects.add(chunk_index as usize);
        if chunk.is_null() {
            return std::ptr::null()
        }

        let within_chunk_index = (index % 65536) * 24;
        let item_ptr = (chunk.add(within_chunk_index as usize)) as *const *const UObject;
        
        *item_ptr
    }

    pub fn len(&self) -> u32 {
        self.num_elements
    }
}

#[repr(C)]
pub struct TArray<T> {
    data: *const T,
    count: u32,
    max: u32
}

impl <T: Copy> TArray<T> {
    pub fn new() -> Self {
        Self {
            data: std::ptr::null(),
            count: 0u32,
            max: 0u32
        }
    }

    pub unsafe fn get(&self, index: u32) -> T {
        let result = self.data.add(index as usize);
        *result
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[repr(C)]
pub struct FString(TArray<u16>);

// bad
pub static mut G_OBJECTS: Option<*const TUObjectArray> = None;
pub static mut G_WORLD: Option<*const *const sdk::UWorld> = None;
pub static mut G_NAMES: Option<*const FNamePool> = None;

#[repr(C)]
pub struct UObject {
    pub vf_table: *const *const u64,
    pub object_flags: u32,
    pub internal_index: u32,
    pub class: *const UClass,
    pub name: FName,
    pub outer: *const UObject
}

impl UObject {
    pub unsafe fn get_name(&self) -> String {
        self.name.get_name()
    }

    pub unsafe fn get_full_name(&self) -> String {
        let mut name: String = String::new();        
        let mut outer = self.outer;

        while !outer.is_null() {
            let outer_name = (*outer).get_name();
            name = format!("{}.{}", outer_name, name);
            outer = (*outer).outer;
        }

        let obj_name = self.get_name();
        let class_name = (*self.class).struct_.field_.object_.get_name();
        name = format!("{} {}", class_name, name);
        name.push_str(&obj_name);

        name
    }

    pub fn is_a(&self) -> bool {
        todo!()
    }

    pub unsafe fn process_event(&self, function: *const usize, params: *mut usize) {
        type VTableFn = extern "C" fn(*const UObject, *const usize, *const usize);
        let self_ptr = self as  *const _  as  *const *const VTableFn;
        let vtable = *self_ptr;
        let fn_call = *vtable.add(76);
        fn_call(self, function, params);     
    }
}