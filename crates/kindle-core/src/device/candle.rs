use serde::{Deserialize, Serialize};

macro_rules! candle_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $device $(<const $n: usize>)?;
    };
}

candle_device!(CandleCudaDevice, Cuda, N);
candle_device!(CandleMetalDevice, Metal, N);
candle_device!(CandleCpuDevice, Cpu);

// Look into changing to a pub visibility identifier for CandleFloatElement and CandleIntElement

#[cfg(not(feature = "autodiff"))]
macro_rules! impl_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
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

#[cfg(not(feature = "autodiff"))]
impl_device!(CandleCudaDevice, Cuda, N);
#[cfg(not(feature = "autodiff"))]
impl_device!(CandleMetalDevice, Metal, N);
#[cfg(not(feature = "autodiff"))]
impl_device!(CandleCpuDevice, Cpu);
