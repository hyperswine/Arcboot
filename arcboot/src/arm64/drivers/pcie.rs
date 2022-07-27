pub struct PCIeDevice {
    id: u64,
    base_addr: u64,
    pcie_root_complex_addr: u64,
}

impl PCIeDevice {
    pub fn new(id: u64, base_addr: u64, pcie_root_complex_addr: u64) -> Self {
        Self {
            id,
            base_addr,
            pcie_root_complex_addr,
        }
    }

    /// Send a read request to this device, requesting to read N bytes
    pub fn read(&mut self, n_bytes: u32) {
        // package the addr?? Wait no send
        // wait what, you need the MMIO addr dont you
        // write the addr is mmio
        // let packet = n_bytes.to_le_bytes();

        unsafe { core::ptr::write_volatile(self.pcie_root_complex_addr as *mut u32, n_bytes) }
    }

    /// Send a write request to this device, requesting to write N bytes
    pub fn write(&mut self, bytes: &[u8]) {
        // write bytes to base_addr. Note when you cast it to an 8 bit pointer, do you lose the precision of the addr?
        unsafe {
            for byte in bytes {
                core::ptr::write_volatile(self.base_addr as *mut u8, *byte)
            }
        }
    }

    /// DMA a buffer from some arbitrary address (could be user or kernel vmem) to the device's buffer
    pub fn dma_from(addr: u64, size: u64) {
        // send an MMIO request to the root complex telling the device to become bus master

        // bus master granted, tell the device the start addr and size with a write packet (?)
        // the device responds by sending write packets to the root complex to the RAM, in 4K chunks
    }
}
