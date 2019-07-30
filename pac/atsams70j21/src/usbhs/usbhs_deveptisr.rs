#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_DEVEPTISR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXINI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXOUTI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSTPI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type NAKOUTI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type NAKINI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERFI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type STALLEDI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SHORTPACKET_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `DTSEQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTSEQR {
    #[doc = "Data0 toggle sequence"]
    DATA0,
    #[doc = "Data1 toggle sequence"]
    DATA1,
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    DATA2,
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    MDATA,
}
impl crate::ToBits<u8> for DTSEQR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DTSEQR::DATA0 => 0,
            DTSEQR::DATA1 => 1,
            DTSEQR::DATA2 => 2,
            DTSEQR::MDATA => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DTSEQ_R = crate::FR<u8, DTSEQR>;
impl DTSEQ_R {
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DTSEQR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DTSEQR::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DTSEQR::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == DTSEQR::MDATA
    }
}
#[doc = "Possible values of the field `NBUSYBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBUSYBKR {
    #[doc = "0 busy bank (all banks free)"]
    _0_BUSY,
    #[doc = "1 busy bank"]
    _1_BUSY,
    #[doc = "2 busy banks"]
    _2_BUSY,
    #[doc = "3 busy banks"]
    _3_BUSY,
}
impl crate::ToBits<u8> for NBUSYBKR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NBUSYBKR::_0_BUSY => 0,
            NBUSYBKR::_1_BUSY => 1,
            NBUSYBKR::_2_BUSY => 2,
            NBUSYBKR::_3_BUSY => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NBUSYBK_R = crate::FR<u8, NBUSYBKR>;
impl NBUSYBK_R {
    #[doc = "Checks if the value of the field is `_0_BUSY`"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        *self == NBUSYBKR::_0_BUSY
    }
    #[doc = "Checks if the value of the field is `_1_BUSY`"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        *self == NBUSYBKR::_1_BUSY
    }
    #[doc = "Checks if the value of the field is `_2_BUSY`"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        *self == NBUSYBKR::_2_BUSY
    }
    #[doc = "Checks if the value of the field is `_3_BUSY`"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        *self == NBUSYBKR::_3_BUSY
    }
}
#[doc = "Possible values of the field `CURRBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRBKR {
    #[doc = "Current bank is bank0"]
    BANK0,
    #[doc = "Current bank is bank1"]
    BANK1,
    #[doc = "Current bank is bank2"]
    BANK2,
}
impl crate::ToBits<u8> for CURRBKR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CURRBKR::BANK0 => 0,
            CURRBKR::BANK1 => 1,
            CURRBKR::BANK2 => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CURRBK_R = crate::FR<u8, CURRBKR>;
impl CURRBK_R {
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == CURRBKR::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == CURRBKR::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == CURRBKR::BANK2
    }
}
#[doc = r"Reader of the field"]
pub type RWALL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CTRLDIR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CFGOK_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BYCT_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacket(&self) -> SHORTPACKET_R {
        SHORTPACKET_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RWALL_R {
        RWALL_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline(always)]
    pub fn byct(&self) -> BYCT_R {
        BYCT_R::new(((self.bits() >> 20) & 0x07ff) as u16)
    }
}