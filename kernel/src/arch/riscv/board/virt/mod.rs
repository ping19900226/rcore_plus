use crate::drivers::*;
use crate::memory::phys_to_virt;
use riscv::register::sie;

/// Enable external interrupt
pub unsafe fn init_external_interrupt() {
    sie::set_sext();
}

pub fn init(dtb: usize) {
    serial::uart16550::driver_init();
    bus::virtio_mmio::driver_init();
    irq::plic::driver_init();
    rtc::rtc_goldfish::driver_init();
    device_tree::init(dtb);
}
