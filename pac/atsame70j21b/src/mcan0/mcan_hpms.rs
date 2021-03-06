#[doc = "Reader of register MCAN_HPMS"]
pub type R = crate::R<u32, super::MCAN_HPMS>;
#[doc = "Reader of field `BIDX`"]
pub type BIDX_R = crate::R<u8, u8>;
#[doc = "Message Storage Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSI_A {
    #[doc = "0: No FIFO selected."]
    NO_FIFO_SEL = 0,
    #[doc = "1: FIFO message lost."]
    LOST = 1,
    #[doc = "2: Message stored in FIFO 0."]
    FIFO_0 = 2,
    #[doc = "3: Message stored in FIFO 1."]
    FIFO_1 = 3,
}
impl From<MSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSI`"]
pub type MSI_R = crate::R<u8, MSI_A>;
impl MSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSI_A {
        match self.bits {
            0 => MSI_A::NO_FIFO_SEL,
            1 => MSI_A::LOST,
            2 => MSI_A::FIFO_0,
            3 => MSI_A::FIFO_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_FIFO_SEL`"]
    #[inline(always)]
    pub fn is_no_fifo_sel(&self) -> bool {
        *self == MSI_A::NO_FIFO_SEL
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == MSI_A::LOST
    }
    #[doc = "Checks if the value of the field is `FIFO_0`"]
    #[inline(always)]
    pub fn is_fifo_0(&self) -> bool {
        *self == MSI_A::FIFO_0
    }
    #[doc = "Checks if the value of the field is `FIFO_1`"]
    #[inline(always)]
    pub fn is_fifo_1(&self) -> bool {
        *self == MSI_A::FIFO_1
    }
}
#[doc = "Reader of field `FIDX`"]
pub type FIDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `FLST`"]
pub type FLST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
