use stm32f401x::*;

pub const DBGMCU: *mut DBGMCU_TypeDef = DBGMCU_BASE as *mut DBGMCU_TypeDef;

pub enum TraceMode {
    Async = 0,
    SyncTraceData1 = DBGMCU_CR_TRACE_MODE_0,
    SyncTraceData2 = DBGMCU_CR_TRACE_MODE_1,
    SyncTraceData4 = DBGMCU_CR_TRACE_MODE_0 | DBGMCU_CR_TRACE_MODE_1,
}

pub enum TraceIO {
    Disable = 0,
    Enable = DBGMCU_CR_TRACE_IOEN,
}

pub enum DebugStandBy {
    Disable = 0,
    Enable = DBGMCU_CR_DBG_STANDBY,
}

pub enum DebugStop {
    Disable = 0,
    Enable = DBGMCU_CR_DBG_STOP,
}

pub enum DebugSleep {
    Disable = 0,
    Enable = DBGMCU_CR_DBG_SLEEP,
}

pub mod CR {
    pub fn set(
        tr: TraceMode,
        io: TraceIO,
        dbgstandby: DebugStandby,
        dbgstop: DebugStop,
        dbgsleep: DebugSleep,
    ) {
        unsafe {
            (*DBGMCU).CR =
                (tr as u32) |
                (io as u32) |
                (dbgstandby as u32) |
                (dbgstop as u32) |
                (dbgsleep as u32);
        }
    }
}
