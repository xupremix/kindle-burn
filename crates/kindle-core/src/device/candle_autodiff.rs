macro_rules! impl_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        impl<$(const $n: usize,)?>
            crate::device::KindleDevice<
                crate::backend::Autodiff<
                    crate::backend::Candle,
                >
            > for crate::device::$device $(<$n>)?
        {
            fn to_device() -> crate::backend::candle::CandleDevice {
                crate::backend::candle::CandleDevice::$device_variant $(($n))?
            }
        }
    };
}

impl_device!(CandleCudaDevice, Cuda, N);
impl_device!(CandleMetalDevice, Metal, N);
impl_device!(CandleCpuDevice, Cpu);
