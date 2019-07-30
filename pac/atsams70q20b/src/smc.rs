#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register"]
    pub smc_cs_number: [SMC_CS_NUMBER; 4],
    _reserved1: [u8; 64usize],
    #[doc = "0x80 - SMC Off-Chip Memory Scrambling Register"]
    pub smc_ocms: SMC_OCMS,
    #[doc = "0x84 - SMC Off-Chip Memory Scrambling KEY1 Register"]
    pub smc_key1: SMC_KEY1,
    #[doc = "0x88 - SMC Off-Chip Memory Scrambling KEY2 Register"]
    pub smc_key2: SMC_KEY2,
    _reserved4: [u8; 88usize],
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    pub smc_wpmr: SMC_WPMR,
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    pub smc_wpsr: SMC_WPSR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register"]
    pub smc_setup: self::smc_cs_number::SMC_SETUP,
    #[doc = "0x04 - SMC Pulse Register"]
    pub smc_pulse: self::smc_cs_number::SMC_PULSE,
    #[doc = "0x08 - SMC Cycle Register"]
    pub smc_cycle: self::smc_cs_number::SMC_CYCLE,
    #[doc = "0x0c - SMC Mode Register"]
    pub smc_mode: self::smc_cs_number::SMC_MODE,
}
#[doc = r"Register block"]
#[doc = "SMC Setup Register"]
pub mod smc_cs_number;
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub struct SMC_OCMS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub mod smc_ocms;
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub struct SMC_KEY1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub mod smc_key1;
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub struct SMC_KEY2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub mod smc_key2;
#[doc = "SMC Write Protection Mode Register"]
pub struct SMC_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SMC Write Protection Mode Register"]
pub mod smc_wpmr;
#[doc = "SMC Write Protection Status Register"]
pub struct SMC_WPSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SMC Write Protection Status Register"]
pub mod smc_wpsr;