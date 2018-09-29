use stm32f401x::*;

pub const DBGMCU: *mut DBGMCU_TypeDef = DBGMCU_BASE as *mut DBGMCU_TypeDef;

#[repr(u32)]
pub enum TraceMode {
    Async = 0,
    SyncTraceData1 = DBGMCU_CR_TRACE_MODE_0,
    SyncTraceData2 = DBGMCU_CR_TRACE_MODE_1,
    SyncTraceData4 = DBGMCU_CR_TRACE_MODE_0 | DBGMCU_CR_TRACE_MODE_1,
}

#[repr(u32)]
pub enum TraceIO {
    Disable = 0,
    Enable = DBGMCU_CR_TRACE_IOEN,
}

#[repr(u32)]
pub enum DebugStandby {
    Disable = 0,
    Enable = DBGMCU_CR_DBG_STANDBY,
}

#[repr(u32)]
pub enum DebugStop {
    Disable = 0,
    Enable = DBGMCU_CR_DBG_STOP,
}

#[repr(u32)]
pub enum DebugSleep {
    Disable = 0,
    Enable = DBGMCU_CR_DBG_SLEEP,
}

pub mod CR {
    use super::*;

    #[naked]
    #[inline(always)]
    pub fn set(
        tr: TraceMode,
        io: TraceIO,
        dbgstandby: DebugStandby,
        dbgstop: DebugStop,
        dbgsleep: DebugSleep,
    ) {
        unsafe {
            (*DBGMCU).CR = (tr as u32)
                | (io as u32)
                | (dbgstandby as u32)
                | (dbgstop as u32)
                | (dbgsleep as u32);
        }
    }
}
