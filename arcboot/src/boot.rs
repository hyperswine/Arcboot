use core::arch::asm;

use alloc::vec::Vec;
use goblin::{
    container::{Container, Ctx},
    elf::{program_header, Elf, ProgramHeader},
};

use crate::memory::map_segment;

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

    // let arc_entry = elf.header.e_entry;

    // jump to header.entry
    // would be "j" on riscv/x86
    // * use an instruction barrier here
    unsafe { asm!("br {arc_entry}", arc_entry = inout(reg) elf.header.e_entry) }
}

#[test]
fn test_load_kernel() {
    // Load an actual file into a vector of bytes
    let kernel_img = [0 as u8; 64];
    load_kernel(&kernel_img)
}
