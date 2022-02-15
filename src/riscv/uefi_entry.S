efi_main:
    # create efi system table structs in memory
        # something about firmware version
    # load efi system table
        # something about bootloader's view of the BIOS and drives

load_quantos:
    # found quantos partition img

    # setup paging
        # 2^52 / 4K = N pages
        # RAM / 4K = M frames
        # kernel virtual addresses (includes drivers, syscall ABI, any kernel MMIO) = 0x0 - 0x1dddddddd (first 8GB)
        # program execution virtual addresses = 0x1ddddddde - 0xffcb9e57d4000 (stack, heap, bss)

    # load kernel img into memory
        # drive number
        # number of sectors to read (ssd)
        # starting sector number / GPT entry
        # cylinder number?
        # some instruction to read from disk into memory (but dont execute)
    
    # transfer to _start (kernel main())
    j _start
