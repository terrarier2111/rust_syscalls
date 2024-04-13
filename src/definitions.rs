#[repr(C)]
pub(crate) struct UnicodeString {
    pub(crate) len: u16,
    pub(crate) max_len: u16,
    pub(crate) buffer: *const (),
}

#[repr(C)]
pub(crate) struct PEB {
    _reserved_0: [u8; 2],
    pub(crate) being_debugged: u8,
    _reserved_2: [u8; 1],
    _reserved_3: [*const (); 2],
    pub(crate) ldr: *const Ldr,
    // ...some other fields
}

#[repr(C)]
pub(crate) struct Ldr {
    _reserved_0: [u8; 8],
    _reserved_1: [*const (); 3],
    pub(crate) in_memory_order_module_list: ListEntry,
}

#[repr(C)]
pub(crate) struct ListEntry {
    pub(crate) f_link: *const ListEntry,
    pub(crate) b_link: *const ListEntry,
}

#[repr(C)]
pub(crate) struct LdrDataTableEntry {
    _reserved_0: [*const (); 2],
    pub(crate) in_memory_order_links: ListEntry,
    _reserved_1: [*const (); 2],
    pub(crate) dll_base: *const (),
    pub(crate) entry_point: *const (),
    _reserved_2: [*const (); 1],
    pub(crate) full_dll_name: UnicodeString,
    _reserved_3: [u8; 8],
    _reserved_4: [*const (); 3],
    pub(crate) checksum: Checksum,
    pub(crate) time_stamp: u32,
}

pub(crate) union Checksum {
    pub(crate) checksum: u32,
    _reserved: *const (),
}

#[repr(C)]
pub(crate) struct ImageDosHeader {
    pub(crate) magic: u16,
    pub(crate) cblp: u16,
    pub(crate) cp: u16,
    pub(crate) crlc: u16,
    pub(crate) cparhdr: u16,
    pub(crate) minalloc: u16,
    pub(crate) maxalloc: u16,
    pub(crate) ss: u16,
    pub(crate) sp: u16,
    pub(crate) csum: u16,
    pub(crate) ip: u16,
    pub(crate) cs: u16,
    pub(crate) lfarlc: u16,
    pub(crate) ovno: u16,
    _reserved_0: [u16; 4],
    pub(crate) oemid: u16,
    pub(crate) oeminfo: u16,
    _reserved_1: [u16; 10],
    pub(crate) lfanew: u32,
}

#[repr(C)]
pub(crate) struct ImageNtHeaders64 {
    pub(crate) signature: u32,
    pub(crate) file_header: ImageFileHeader,
    pub(crate) optional_header: ImageOptionalHeader64,
}

#[repr(C)]
pub(crate) struct ImageNtHeaders32 {
    pub(crate) signature: u32,
    pub(crate) file_header: ImageFileHeader,
    pub(crate) optional_header: ImageOptionalHeader32,
}

const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;

#[repr(C)]
pub(crate) struct ImageOptionalHeader32 {
    pub(crate) magic: u16,
    pub(crate) major_linker_version: u8,
    pub(crate) minor_linker_version: u8,
    pub(crate) size_of_code: u32,
    pub(crate) size_of_initialized_data: u32,
    pub(crate) size_of_uninitialized_data: u32,
    pub(crate) address_of_entry_point: u32,
    pub(crate) base_of_code: u32,
    pub(crate) base_of_data: u32,
    pub(crate) image_base: u32,
    pub(crate) section_alignment: u32,
    pub(crate) file_alignment: u32,
    pub(crate) major_operating_system_version: u16,
    pub(crate) minor_operating_system_version: u16,
    pub(crate) major_image_version: u16,
    pub(crate) minor_image_version: u16,
    pub(crate) major_subsystem_version: u16,
    pub(crate) minor_subsystem_version: u16,
    pub(crate) win32_version_value: u32,
    pub(crate) size_of_image: u32,
    pub(crate) size_of_headers: u32,
    pub(crate) check_sum: u32,
    pub(crate) subsystem: u16,
    pub(crate) dll_characteristics: u16,
    pub(crate) size_of_stack_reserve: u32,
    pub(crate) size_of_stack_commit: u32,
    pub(crate) size_of_heap_reserve: u32,
    pub(crate) size_of_heap_commit: u32,
    pub(crate) loader_flags: u32,
    pub(crate) numver_if_rva_and_sizes: u32,
    pub(crate) data_directory: [ImageDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

#[repr(C)]
pub(crate) struct ImageOptionalHeader64 {
    pub(crate) magic: u16,
    pub(crate) major_linker_version: u8,
    pub(crate) minor_linker_version: u8,
    pub(crate) size_of_code: u32,
    pub(crate) size_of_initialized_data: u32,
    pub(crate) size_of_uninitialized_data: u32,
    pub(crate) address_of_entry_point: u32,
    pub(crate) base_of_code: u32,
    pub(crate) image_base: u64,
    pub(crate) section_alignment: u32,
    pub(crate) file_alignment: u32,
    pub(crate) major_operating_system_version: u16,
    pub(crate) minor_operating_system_version: u16,
    pub(crate) major_image_version: u16,
    pub(crate) minor_image_version: u16,
    pub(crate) major_subsystem_version: u16,
    pub(crate) minor_subsystem_version: u16,
    pub(crate) win32_version_value: u32,
    pub(crate) size_of_image: u32,
    pub(crate) size_of_headers: u32,
    pub(crate) check_sum: u32,
    pub(crate) subsystem: u16,
    pub(crate) dll_characteristics: u16,
    pub(crate) size_of_stack_reserve: u64,
    pub(crate) size_of_stack_commit: u64,
    pub(crate) size_of_heap_reserve: u64,
    pub(crate) size_of_heap_commit: u64,
    pub(crate) loader_flags: u32,
    pub(crate) numver_if_rva_and_sizes: u32,
    pub(crate) data_directory: [ImageDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

#[repr(C)]
pub(crate) struct ImageDataDirectory {
    pub(crate) virtual_address: u32,
    pub(crate) size: u32,
}

#[cfg(target_pointer_width = "64")]
pub(crate) type ImageNtHeaders = ImageNtHeaders64;
#[cfg(target_pointer_width = "32")]
pub(crate) type ImageNtHeaders = ImageNtHeaders32;

#[repr(C)]
pub(crate) struct ImageFileHeader {
    pub(crate) machine: u16,
    pub(crate) number_of_sections: u16,
    pub(crate) time_stamp: u32,
    pub(crate) ptr_to_symbol_table: u32,
    pub(crate) number_of_symbols: u32,
    pub(crate) size_of_optional_header: u16,
    pub(crate) characteristics: u16,
}

#[repr(C)]
pub(crate) struct ImageExportDirectory {
    pub(crate) characteristics: u32,
    pub(crate) time_stamp: u32,
    pub(crate) major_version: u16,
    pub(crate) minor_version: u16,
    pub(crate) name: u32,
    pub(crate) base: u32,
    pub(crate) number_of_functions: u32,
    pub(crate) number_of_names: u32,
    pub(crate) address_of_functions: u32,
    pub(crate) address_of_names: u32,
    pub(crate) address_of_names_ordinals: u32,
}
