#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub us_cr: US_CR,
    #[doc = "0x04 - Mode Register"]
    pub us_mr: US_MR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub us_ier: US_IER,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub us_idr: US_IDR,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub us_imr: US_IMR,
    #[doc = "0x14 - Channel Status Register"]
    pub us_csr: US_CSR,
    #[doc = "0x18 - Receive Holding Register"]
    pub us_rhr: US_RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub us_thr: US_THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub us_brgr: US_BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub us_rtor: US_RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub us_ttgr: US_TTGR,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub us_fidi: US_FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub us_ner: US_NER,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub us_if: US_IF,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub us_man: US_MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub us_linmr: US_LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub us_linir: US_LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub us_linbrr: US_LINBRR,
    #[doc = "0x60 - LON Mode Register"]
    pub us_lonmr: US_LONMR,
    #[doc = "0x64 - LON Preamble Register"]
    pub us_lonpr: US_LONPR,
    #[doc = "0x68 - LON Data Length Register"]
    pub us_londl: US_LONDL,
    #[doc = "0x6c - LON L2HDR Register"]
    pub us_lonl2hdr: US_LONL2HDR,
    #[doc = "0x70 - LON Backlog Register"]
    pub us_lonbl: US_LONBL,
    #[doc = "0x74 - LON Beta1 Tx Register"]
    pub us_lonb1tx: US_LONB1TX,
    #[doc = "0x78 - LON Beta1 Rx Register"]
    pub us_lonb1rx: US_LONB1RX,
    #[doc = "0x7c - LON Priority Register"]
    pub us_lonprio: US_LONPRIO,
    #[doc = "0x80 - LON IDT Tx Register"]
    pub us_idttx: US_IDTTX,
    #[doc = "0x84 - LON IDT Rx Register"]
    pub us_idtrx: US_IDTRX,
    #[doc = "0x88 - IC DIFF Register"]
    pub us_icdiff: US_ICDIFF,
    _reserved29: [u8; 88usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub us_wpmr: US_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub us_wpsr: US_WPSR,
}
#[doc = "Control Register"]
pub struct US_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod us_cr;
#[doc = "Mode Register"]
pub struct US_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod us_mr;
#[doc = "Interrupt Enable Register"]
pub struct US_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod us_ier;
#[doc = "Interrupt Disable Register"]
pub struct US_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod us_idr;
#[doc = "Interrupt Mask Register"]
pub struct US_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod us_imr;
#[doc = "Channel Status Register"]
pub struct US_CSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod us_csr;
#[doc = "Receive Holding Register"]
pub struct US_RHR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive Holding Register"]
pub mod us_rhr;
#[doc = "Transmit Holding Register"]
pub struct US_THR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod us_thr;
#[doc = "Baud Rate Generator Register"]
pub struct US_BRGR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator Register"]
pub mod us_brgr;
#[doc = "Receiver Time-out Register"]
pub struct US_RTOR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receiver Time-out Register"]
pub mod us_rtor;
#[doc = "Transmitter Timeguard Register"]
pub struct US_TTGR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Timeguard Register"]
pub mod us_ttgr;
#[doc = "FI DI Ratio Register"]
pub struct US_FIDI {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FI DI Ratio Register"]
pub mod us_fidi;
#[doc = "Number of Errors Register"]
pub struct US_NER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Number of Errors Register"]
pub mod us_ner;
#[doc = "IrDA Filter Register"]
pub struct US_IF {
    register: vcell::VolatileCell<u32>,
}
#[doc = "IrDA Filter Register"]
pub mod us_if;
#[doc = "Manchester Configuration Register"]
pub struct US_MAN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Manchester Configuration Register"]
pub mod us_man;
#[doc = "LIN Mode Register"]
pub struct US_LINMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LIN Mode Register"]
pub mod us_linmr;
#[doc = "LIN Identifier Register"]
pub struct US_LINIR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LIN Identifier Register"]
pub mod us_linir;
#[doc = "LIN Baud Rate Register"]
pub struct US_LINBRR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LIN Baud Rate Register"]
pub mod us_linbrr;
#[doc = "LON Mode Register"]
pub struct US_LONMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Mode Register"]
pub mod us_lonmr;
#[doc = "LON Preamble Register"]
pub struct US_LONPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Preamble Register"]
pub mod us_lonpr;
#[doc = "LON Data Length Register"]
pub struct US_LONDL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Data Length Register"]
pub mod us_londl;
#[doc = "LON L2HDR Register"]
pub struct US_LONL2HDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON L2HDR Register"]
pub mod us_lonl2hdr;
#[doc = "LON Backlog Register"]
pub struct US_LONBL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Backlog Register"]
pub mod us_lonbl;
#[doc = "LON Beta1 Tx Register"]
pub struct US_LONB1TX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Beta1 Tx Register"]
pub mod us_lonb1tx;
#[doc = "LON Beta1 Rx Register"]
pub struct US_LONB1RX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Beta1 Rx Register"]
pub mod us_lonb1rx;
#[doc = "LON Priority Register"]
pub struct US_LONPRIO {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON Priority Register"]
pub mod us_lonprio;
#[doc = "LON IDT Tx Register"]
pub struct US_IDTTX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON IDT Tx Register"]
pub mod us_idttx;
#[doc = "LON IDT Rx Register"]
pub struct US_IDTRX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LON IDT Rx Register"]
pub mod us_idtrx;
#[doc = "IC DIFF Register"]
pub struct US_ICDIFF {
    register: vcell::VolatileCell<u32>,
}
#[doc = "IC DIFF Register"]
pub mod us_icdiff;
#[doc = "Write Protection Mode Register"]
pub struct US_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod us_wpmr;
#[doc = "Write Protection Status Register"]
pub struct US_WPSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod us_wpsr;