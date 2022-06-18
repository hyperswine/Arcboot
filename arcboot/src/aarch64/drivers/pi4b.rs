// NOTE: with edk2 we should be able to do things
// prob more easily?
// like it shouldnt load our image onto 0x80000
// it might be configurable
// and load an EFI binary from a certain folder

// so we dont care about that
// what we do care about is the memory map
// how does runtime UEFI set it up? if its just ID mapped, then maybe we just assume such

// can i link acpica? but then i still have to implement the above
// just makes it more uniform i guess
// idk idk idk
// maybe get the stackpointer of EL1 and check
