use serde::{Deserialize, Serialize};

macro_rules! candle_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $device $(<const $n: usize>)?;
        impl<'dv, $(const $n: usize,)?>
            crate::device::KindleDevice<
                'dv,
                crate::backend::Candle,
            > for $device $(<$n>)?
        {
        }
        impl $(<const $n: usize>)? From<$device $(<$n>)?>
            for crate::backend::candle::CandleDevice {
            fn from(_: $device $(<$n>)?) -> Self {
                Self::$device_variant $(($n))?
            }
        }
        impl $(<const $n: usize>)? From<&$device $(<$n>)?>
            for crate::backend::candle::CandleDevice {
            fn from(_: &$device $(<$n>)?) -> Self {
                Self::$device_variant $(($n))?
            }
        }
    };
}

candle_device!(CandleCudaDevice, Cuda, N);
candle_device!(CandleCpuDevice, Cpu);
candle_device!(CandleMetalDevice, Metal, N);

// Look into changing to a pub visibility identifier for CandleFloatElement and CandleIntElement
