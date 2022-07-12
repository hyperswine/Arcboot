use core::{arch::asm, intrinsics::transmute};

use alloc::vec::Vec;
use arcboot_api::{make_default, ArcServices, DefaultServices};
use goblin::{
    container::{Container, Ctx},
    elf::{program_header, Elf, ProgramHeader},
};

use crate::memory::{map_segment, set_stack};

const ELF64_HDR_SIZE: usize = 64;

/// Given a kernel ELF img in bytes, parse and load its segments
/// Then pass off execution to it. Should prob have setup TTBR1 and using those addresses (0xffff__ffff_ffff_ffff - 0x0000_ffff_ffff_ffff)
pub fn load_kernel(kernel_img: &[u8]) {
    // PARSE KERNEL ELF

    let header =
        Elf::parse_header(&kernel_img[..ELF64_HDR_SIZE]).map_err(|_| "parse elf header error");

    let header = match header {
        Ok(h) => h,
        Err(err) => panic!("Error! {err}"),
    };

    let program_hdr_table_size = (header.e_phnum * header.e_phentsize) as usize;
    let program_table = &kernel_img[ELF64_HDR_SIZE..ELF64_HDR_SIZE + program_hdr_table_size];

    let mut elf = Elf::lazy_parse(header)
        .map_err(|_| "cannot parse ELF file")
        .unwrap();

    let ctx = Ctx {
        le: scroll::Endian::Little,
        container: Container::Big,
    };

    // parse and assemble the program headers
    elf.program_headers = match ProgramHeader::parse(
        &program_table,
        header.e_phoff as usize,
        header.e_phnum as usize,
        ctx,
    )
    .map_err(|_| "parse program headers error")
    {
        Ok(r) => r,
        Err(err) => panic!("Error! {err}"),
    };

    let sections: Vec<&[u8]> = elf
        .program_headers
        .iter()
        .map(|h| {
            if h.p_type == program_header::PT_INTERP && h.p_filesz != 0 {
                let offset = h.p_filesz as usize;
                let count = h.p_offset as usize;

                &kernel_img[offset..offset + count]
            } else {
                &[0; 1]
            }
        })
        .collect();

    // LOAD SECTIONS

    // .text should be checked for higher half. It should be higher half in most cases, hopefully they linked it right

    elf.program_headers.iter().enumerate().for_each(|(ind, p)| {
        // segment should usually be 1, 6, 5, 4
        map_segment(sections[ind], p.p_vaddr);
    });

    // set SP = elf.sp?? Nah just set it at 0xffff_ffff_....
    // assume that paging has been setup properly for TTBR1 and we have switched to it. Beforehand, we should have also mapped 64K worth of frames down from 0xffff_...
    set_stack(0xFFFF_FFFF_FFFF_FFFF);

    // Pass ArcServices to the kernel
    // * stack allocated, will disappear normally, so maybe ensure this function doesnt return too early or not at all? Also will be in the bootloader's stack, unless we reset the sp to the kernel's beforehand
    let mut arcservices = make_default();
    let mut point_arc = &mut arcservices as *mut DefaultServices;

    // set a0 = *mut arcservices
    unsafe { asm!("mov a0 {p}", p = inout(reg) point_arc) }

    let arc_entry = elf.header.e_entry as *const ();
    // transmute that addr to a function pointer
    unsafe {
        transmute::<*const (), fn(ArcServices)>(arc_entry)
    }

    // jump to header.entry
    // would be "j" on riscv/x86
    // * prob use an instruction barrier here?

    unsafe { asm!("br {arc_entry}", arc_entry = inout(reg) elf.header.e_entry) }
}

#[test]
fn test_load_kernel() {
    // Load an actual file into a vector of bytes
    let kernel_img = [0 as u8; 64];
    load_kernel(&kernel_img)
}
