use std::arch::asm;
use std::ptr::{addr_of, null};

use core::slice;

use crate::definitions::{
    ImageDataDirectory, ImageDosHeader, ImageExportDirectory, ImageNtHeaders, LdrDataTableEntry,
    ListEntry, PEB,
};
use crate::obf::dbj2_hash;

#[cfg(target_arch = "x86_64")]
unsafe fn __readgsqword(offset: u32) -> u64 {
    let out: u64;
    asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) out,
        in(reg) offset,
        options(nostack, nomem),
    );
    out
}

#[cfg(target_arch = "x86")]
unsafe fn __readfsdword(offset: u32) -> u32 {
    let out: u32;
    asm!(
        "mov {:e}, fs:[{:e}]",
        lateout(reg) out,
        in(reg) offset,
        options(nostack, nomem),
    );
    out
}

#[cfg(target_arch = "x86")]
pub unsafe fn is_wow64() -> bool {
    let addr = __readfsdword(0xC0);
    addr != 0
}

#[inline(always)]
#[cfg(target_pointer_width = "64")]
fn get_peb() -> *const PEB {
    let peb;
    unsafe {
        asm!("mov {}, gs:0x60", out(reg) peb, options(nomem, nostack));
    }
    peb
}

#[inline(always)]
#[cfg(target_pointer_width = "32")]
fn get_peb() -> *const PEB {
    let peb;
    unsafe {
        asm!("mov {}, fs:0x30", out(reg) peb, options(nomem, nostack));
    }
    peb
}

fn get_cstr_len(pointer: *const char) -> usize {
    let mut tmp = pointer as u64;
    unsafe {
        while *(tmp as *const u8) != 0 {
            tmp += 1;
        }
    }
    (tmp - pointer as u64) as _
}

fn get_module_addr(hash: u32) -> *const () {
    let mut dt_entry;
    let mut mod_hash;
    let mut mod_name;
    let mut mod_len;

    unsafe {
        let ldr = (*get_peb()).ldr;
        let header = addr_of!((*ldr).in_memory_order_module_list) as *const ListEntry;
        let mut entry = (*header).f_link;

        while header as u64 != entry as u64 {
            dt_entry = entry.cast::<LdrDataTableEntry>();
            mod_len = ((*dt_entry).full_dll_name.len) as usize;
            mod_name =
                slice::from_raw_parts((*dt_entry).full_dll_name.buffer as *const u8, mod_len);
            mod_hash = dbj2_hash(mod_name);

            if mod_hash == hash {
                return (*dt_entry).dll_base;
            }

            entry = (*entry).f_link;
        }
    }
    null()
}

fn get_function_addr(mdoule_addr: *const (), hash: u32) -> *const () {
    let dos_header = mdoule_addr as *const ImageDosHeader;

    unsafe {
        let nt_header =
            (dos_header as usize + (*dos_header).lfanew as usize) as *const ImageNtHeaders;
        let data_dir =
            addr_of!((*nt_header).optional_header.data_directory[0]) as *const ImageDataDirectory;

        if (*data_dir).virtual_address != 0 {
            let exp_dir = (dos_header as usize + (*data_dir).virtual_address as usize)
                as *const ImageExportDirectory;
            let addr_funcs =
                (dos_header as usize + (*exp_dir).address_of_functions as usize) as *const ();
            let addr_names =
                (dos_header as usize + (*exp_dir).address_of_names as usize) as *const ();
            let addr_ords =
                (dos_header as usize + (*exp_dir).address_of_names_ordinals as usize) as *const ();

            let name_list = slice::from_raw_parts(
                addr_names as *const u32,
                (*exp_dir).number_of_names as usize,
            );
            let ord_list =
                slice::from_raw_parts(addr_ords as *const u16, (*exp_dir).number_of_names as usize);
            let addr_list = slice::from_raw_parts(
                addr_funcs as *const u32,
                (*exp_dir).number_of_functions as usize,
            );

            let mut str_addr;
            let mut str_len;
            for iter in 0..(*exp_dir).number_of_names as usize {
                str_addr = dos_header as usize + name_list[iter] as usize;
                str_len = get_cstr_len(str_addr as _);
                if hash == dbj2_hash(slice::from_raw_parts(str_addr as _, str_len)) {
                    return (dos_header as usize + addr_list[ord_list[iter] as usize] as usize)
                        as *const ();
                }
            }
        }
    }
    null()
}

#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "direct", not(feature = "indirect")))]
pub fn get_ssn(hash: u32) -> u16 {
    let ntdll_addr = get_module_addr(crate::obf!("ntdll.dll"));
    let funct_addr = get_function_addr(ntdll_addr, hash);
    unsafe { *((funct_addr as u64 + 4) as *const u16) }
}

#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "indirect", not(feature = "direct")))]
pub fn get_ssn(hash: u32) -> (u16, u64) {
    let ntdll_addr = get_module_addr(crate::obf!("ntdll.dll"));
    let funct_addr = get_function_addr(ntdll_addr, hash);
    let ssn = unsafe { *((funct_addr as u64 + 4) as *const u16) };
    let ssn_addr = funct_addr as u64 + 0x12;

    (ssn, ssn_addr)
}

#[cfg(target_arch = "x86")]
#[cfg(all(feature = "direct", not(feature = "indirect")))]
pub fn get_ssn(hash: u32) -> u16 {
    let ntdll_addr = get_module_addr(crate::obf!("ntdll.dll"));
    let funct_addr = get_function_addr(ntdll_addr, hash);
    unsafe { *((funct_addr as u64 + 1) as *const u16) }
}

#[cfg(target_arch = "x86")]
#[cfg(all(feature = "indirect", not(feature = "direct")))]
pub fn get_ssn(hash: u32) -> (u16, u32) {
    let ssn_addr: u32;
    let ssn: u16;

    let ntdll_addr = get_module_addr(crate::obf!("ntdll.dll"));
    let funct_addr = get_function_addr(ntdll_addr, hash);
    unsafe {
        ssn = *((funct_addr as u64 + 1) as *const u16);

        if is_wow64() {
            ssn_addr = funct_addr as u32 + 0x0A;
        } else {
            ssn_addr = funct_addr as u32 + 0x0F;
        }
    }
    (ssn, ssn_addr)
}
