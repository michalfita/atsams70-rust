#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISI_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DIS_DONE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SRST_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CDC_PND_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type VSYNC_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PXFR_DONE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CXFR_DONE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SIP_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P_OVR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type C_OVR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CRC_ERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FR_OVR_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn dis_done(&self) -> DIS_DONE_R {
        DIS_DONE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Module Software Reset Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending Codec Request"]
    #[inline(always)]
    pub fn cdc_pnd(&self) -> CDC_PND_R {
        CDC_PND_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization (cleared on read)"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PXFR_DONE_R {
        PXFR_DONE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CXFR_DONE_R {
        CXFR_DONE_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Synchronization in Progress"]
    #[inline(always)]
    pub fn sip(&self) -> SIP_R {
        SIP_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn p_ovr(&self) -> P_OVR_R {
        P_OVR_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn c_ovr(&self) -> C_OVR_R {
        C_OVR_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error (cleared on read)"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun (cleared on read)"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FR_OVR_R {
        FR_OVR_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}