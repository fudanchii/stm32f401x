use stm32f401x::*;

const PWR: *mut PWR_TypeDef = PWR_BASE as *mut PWR_TypeDef;

#[repr(u32)]
pub enum VOS {
    Scale2 = PWR_CR_VOS_1,
    Scale3 = PWR_CR_VOS_0,
}

#[repr(u32)]
pub enum PLS {
    Level0 = PWR_CR_PLS_LEV0,
    Level1 = PWR_CR_PLS_LEV1,
    Level2 = PWR_CR_PLS_LEV2,
    Level3 = PWR_CR_PLS_LEV3,
    Level4 = PWR_CR_PLS_LEV4,
    Level5 = PWR_CR_PLS_LEV5,
    Level6 = PWR_CR_PLS_LEV6,
    Level7 = PWR_CR_PLS_LEV7,
}

#[repr(u32)]
pub enum ADCDC1 {
    Unset = 0,
    Set = PWR_CR_ADCDC1,
}

#[repr(u32)]
pub enum MRLVDS {
    Unset = 0,
    Set = PWR_CR_MRLVDS,
}

#[repr(u32)]
pub enum LPLVDS {
    Unset = 0,
    Set = PWR_CR_LPLVDS,
}

#[repr(u32)]
pub enum FPDS {
    Unset = 0,
    Set = PWR_CR_FPDS,
}

#[repr(u32)]
pub enum DBP {
    Unset = 0,
    Set = PWR_CR_DBP,
}

#[repr(u32)]
pub enum PVDE {
    Unset = 0,
    Set = PWR_CR_PVDE,
}

#[repr(u32)]
pub enum CSBF {
    Unset = 0,
    Set = PWR_CR_CSBF,
}

#[repr(u32)]
pub enum CWUF {
    Unset = 0,
    Set = PWR_CR_CWUF,
}

#[repr(u32)]
pub enum PDDS {
    Unset = 0,
    Set = PWR_CR_PDDS,
}

#[repr(u32)]
pub enum LPDS {
    Unset = 0,
    Set = PWR_CR_LPDS,
}

pub mod CR {
    use super::*;

    #[inline(always)]
    pub fn set(
        vos: VOS,
        adcdc1: ADCDC1,
        mrlvds: MRLVDS,
        lplvds: LPLVDS,
        fpds: FPDS,
        dbp: DBP,
        pls: PLS,
        pvde: PVDE,
        csbf: CSBF,
        cwuf: CWUF,
        pdds: PDDS,
        lpds: LPDS,
    ) {
        unsafe {
            (*PWR).CR |= (vos as u32)
                | (adcdc1 as u32)
                | (mrlvds as u32)
                | (lplvds as u32)
                | (fpds as u32)
                | (dbp as u32)
                | (pls as u32)
                | (pvde as u32)
                | (csbf as u32)
                | (cwuf as u32)
                | (pdds as u32)
                | (lpds as u32);
        }
    }
}
