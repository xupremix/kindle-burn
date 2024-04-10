use serde::{Deserialize, Serialize};

macro_rules! candle_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $device $(<const $n: usize = 0>)?;
        impl<$(const $n: usize,)?>
            crate::device::KindleDevice<
                crate::backend::Candle,
            > for $device $(<$n>)?
        {
            fn to_device() -> crate::backend::candle::CandleDevice {
                crate::backend::candle::CandleDevice::$device_variant $(($n))?
            }
        }
    };
}

candle_device!(CandleCudaDevice, Cuda, N);
candle_device!(CandleMetalDevice, Metal, N);
candle_device!(CandleCpuDevice, Cpu);

// Look into changing to a pub visibility identifier for CandleFloatElement and CandleIntElement
