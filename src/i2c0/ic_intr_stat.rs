#[doc = "Reader of register IC_INTR_STAT"]
pub type R = crate::R<u32, super::IC_INTR_STAT>;
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_MASTER_ON_HOLD bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_MASTER_ON_HOLD_A {
    #[doc = "0: R_MASTER_ON_HOLD interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_MASTER_ON_HOLD interrupt is active"]
    ACTIVE = 1,
}
impl From<R_MASTER_ON_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: R_MASTER_ON_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_MASTER_ON_HOLD`"]
pub type R_MASTER_ON_HOLD_R = crate::R<bool, R_MASTER_ON_HOLD_A>;
impl R_MASTER_ON_HOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_MASTER_ON_HOLD_A {
        match self.bits {
            false => R_MASTER_ON_HOLD_A::INACTIVE,
            true => R_MASTER_ON_HOLD_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_MASTER_ON_HOLD_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_MASTER_ON_HOLD_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_RESTART_DET_A {
    #[doc = "0: R_RESTART_DET interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_RESTART_DET interrupt is active"]
    ACTIVE = 1,
}
impl From<R_RESTART_DET_A> for bool {
    #[inline(always)]
    fn from(variant: R_RESTART_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_RESTART_DET`"]
pub type R_RESTART_DET_R = crate::R<bool, R_RESTART_DET_A>;
impl R_RESTART_DET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_RESTART_DET_A {
        match self.bits {
            false => R_RESTART_DET_A::INACTIVE,
            true => R_RESTART_DET_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_RESTART_DET_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_RESTART_DET_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_GEN_CALL_A {
    #[doc = "0: R_GEN_CALL interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_GEN_CALL interrupt is active"]
    ACTIVE = 1,
}
impl From<R_GEN_CALL_A> for bool {
    #[inline(always)]
    fn from(variant: R_GEN_CALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_GEN_CALL`"]
pub type R_GEN_CALL_R = crate::R<bool, R_GEN_CALL_A>;
impl R_GEN_CALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_GEN_CALL_A {
        match self.bits {
            false => R_GEN_CALL_A::INACTIVE,
            true => R_GEN_CALL_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_GEN_CALL_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_GEN_CALL_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_START_DET_A {
    #[doc = "0: R_START_DET interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_START_DET interrupt is active"]
    ACTIVE = 1,
}
impl From<R_START_DET_A> for bool {
    #[inline(always)]
    fn from(variant: R_START_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_START_DET`"]
pub type R_START_DET_R = crate::R<bool, R_START_DET_A>;
impl R_START_DET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_START_DET_A {
        match self.bits {
            false => R_START_DET_A::INACTIVE,
            true => R_START_DET_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_START_DET_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_START_DET_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_STOP_DET_A {
    #[doc = "0: R_STOP_DET interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_STOP_DET interrupt is active"]
    ACTIVE = 1,
}
impl From<R_STOP_DET_A> for bool {
    #[inline(always)]
    fn from(variant: R_STOP_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_STOP_DET`"]
pub type R_STOP_DET_R = crate::R<bool, R_STOP_DET_A>;
impl R_STOP_DET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_STOP_DET_A {
        match self.bits {
            false => R_STOP_DET_A::INACTIVE,
            true => R_STOP_DET_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_STOP_DET_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_STOP_DET_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_ACTIVITY_A {
    #[doc = "0: R_ACTIVITY interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_ACTIVITY interrupt is active"]
    ACTIVE = 1,
}
impl From<R_ACTIVITY_A> for bool {
    #[inline(always)]
    fn from(variant: R_ACTIVITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_ACTIVITY`"]
pub type R_ACTIVITY_R = crate::R<bool, R_ACTIVITY_A>;
impl R_ACTIVITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_ACTIVITY_A {
        match self.bits {
            false => R_ACTIVITY_A::INACTIVE,
            true => R_ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_ACTIVITY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_ACTIVITY_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_RX_DONE_A {
    #[doc = "0: R_RX_DONE interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_RX_DONE interrupt is active"]
    ACTIVE = 1,
}
impl From<R_RX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: R_RX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_RX_DONE`"]
pub type R_RX_DONE_R = crate::R<bool, R_RX_DONE_A>;
impl R_RX_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_RX_DONE_A {
        match self.bits {
            false => R_RX_DONE_A::INACTIVE,
            true => R_RX_DONE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_RX_DONE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_RX_DONE_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_TX_ABRT_A {
    #[doc = "0: R_TX_ABRT interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_TX_ABRT interrupt is active"]
    ACTIVE = 1,
}
impl From<R_TX_ABRT_A> for bool {
    #[inline(always)]
    fn from(variant: R_TX_ABRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_TX_ABRT`"]
pub type R_TX_ABRT_R = crate::R<bool, R_TX_ABRT_A>;
impl R_TX_ABRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_TX_ABRT_A {
        match self.bits {
            false => R_TX_ABRT_A::INACTIVE,
            true => R_TX_ABRT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_TX_ABRT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_TX_ABRT_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_RD_REQ_A {
    #[doc = "0: R_RD_REQ interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_RD_REQ interrupt is active"]
    ACTIVE = 1,
}
impl From<R_RD_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: R_RD_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_RD_REQ`"]
pub type R_RD_REQ_R = crate::R<bool, R_RD_REQ_A>;
impl R_RD_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_RD_REQ_A {
        match self.bits {
            false => R_RD_REQ_A::INACTIVE,
            true => R_RD_REQ_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_RD_REQ_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_RD_REQ_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_TX_EMPTY_A {
    #[doc = "0: R_TX_EMPTY interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_TX_EMPTY interrupt is active"]
    ACTIVE = 1,
}
impl From<R_TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: R_TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_TX_EMPTY`"]
pub type R_TX_EMPTY_R = crate::R<bool, R_TX_EMPTY_A>;
impl R_TX_EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_TX_EMPTY_A {
        match self.bits {
            false => R_TX_EMPTY_A::INACTIVE,
            true => R_TX_EMPTY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_TX_EMPTY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_TX_EMPTY_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_TX_OVER_A {
    #[doc = "0: R_TX_OVER interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_TX_OVER interrupt is active"]
    ACTIVE = 1,
}
impl From<R_TX_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: R_TX_OVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_TX_OVER`"]
pub type R_TX_OVER_R = crate::R<bool, R_TX_OVER_A>;
impl R_TX_OVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_TX_OVER_A {
        match self.bits {
            false => R_TX_OVER_A::INACTIVE,
            true => R_TX_OVER_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_TX_OVER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_TX_OVER_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_RX_FULL_A {
    #[doc = "0: R_RX_FULL interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_RX_FULL interrupt is active"]
    ACTIVE = 1,
}
impl From<R_RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: R_RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_RX_FULL`"]
pub type R_RX_FULL_R = crate::R<bool, R_RX_FULL_A>;
impl R_RX_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_RX_FULL_A {
        match self.bits {
            false => R_RX_FULL_A::INACTIVE,
            true => R_RX_FULL_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_RX_FULL_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_RX_FULL_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_RX_OVER_A {
    #[doc = "0: R_RX_OVER interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: R_RX_OVER interrupt is active"]
    ACTIVE = 1,
}
impl From<R_RX_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: R_RX_OVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_RX_OVER`"]
pub type R_RX_OVER_R = crate::R<bool, R_RX_OVER_A>;
impl R_RX_OVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_RX_OVER_A {
        match self.bits {
            false => R_RX_OVER_A::INACTIVE,
            true => R_RX_OVER_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_RX_OVER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_RX_OVER_A::ACTIVE
    }
}
#[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit.\\n\\n Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R_RX_UNDER_A {
    #[doc = "0: RX_UNDER interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RX_UNDER interrupt is active"]
    ACTIVE = 1,
}
impl From<R_RX_UNDER_A> for bool {
    #[inline(always)]
    fn from(variant: R_RX_UNDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `R_RX_UNDER`"]
pub type R_RX_UNDER_R = crate::R<bool, R_RX_UNDER_A>;
impl R_RX_UNDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_RX_UNDER_A {
        match self.bits {
            false => R_RX_UNDER_A::INACTIVE,
            true => R_RX_UNDER_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == R_RX_UNDER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == R_RX_UNDER_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 13 - See IC_RAW_INTR_STAT for a detailed description of R_MASTER_ON_HOLD bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_master_on_hold(&self) -> R_MASTER_ON_HOLD_R {
        R_MASTER_ON_HOLD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_restart_det(&self) -> R_RESTART_DET_R {
        R_RESTART_DET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_gen_call(&self) -> R_GEN_CALL_R {
        R_GEN_CALL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_start_det(&self) -> R_START_DET_R {
        R_START_DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_stop_det(&self) -> R_STOP_DET_R {
        R_STOP_DET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_activity(&self) -> R_ACTIVITY_R {
        R_ACTIVITY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_rx_done(&self) -> R_RX_DONE_R {
        R_RX_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_tx_abrt(&self) -> R_TX_ABRT_R {
        R_TX_ABRT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_rd_req(&self) -> R_RD_REQ_R {
        R_RD_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_tx_empty(&self) -> R_TX_EMPTY_R {
        R_TX_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_tx_over(&self) -> R_TX_OVER_R {
        R_TX_OVER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_rx_full(&self) -> R_RX_FULL_R {
        R_RX_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_rx_over(&self) -> R_RX_OVER_R {
        R_RX_OVER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit.\\n\\n Reset value: 0x0"]
    #[inline(always)]
    pub fn r_rx_under(&self) -> R_RX_UNDER_R {
        R_RX_UNDER_R::new((self.bits & 0x01) != 0)
    }
}
