use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandleCudaDevice<const N: usize>;
impl<const N: usize> crate::Sealed for CandleCudaDevice<N> {}
impl<const N: usize> crate::device::KindleDevice<'_> for CandleCudaDevice<N> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandleCpuDevice;
impl crate::Sealed for CandleCpuDevice {}
impl crate::device::KindleDevice<'_> for CandleCpuDevice {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandleMetalDevice<const N: usize>;
impl<const N: usize> crate::Sealed for CandleMetalDevice<N> {}
impl<const N: usize> crate::device::KindleDevice<'_> for CandleMetalDevice<N> {}

// Taking owenership
impl<const N: usize> From<CandleCudaDevice<N>> for crate::backend::candle::CandleDevice {
    fn from(_: CandleCudaDevice<N>) -> Self {
        Self::Cuda(N)
    }
}

impl From<CandleCpuDevice> for crate::backend::candle::CandleDevice {
    fn from(_: CandleCpuDevice) -> Self {
        Self::Cpu
    }
}

impl<const N: usize> From<CandleMetalDevice<N>> for crate::backend::candle::CandleDevice {
    fn from(_: CandleMetalDevice<N>) -> Self {
        Self::Metal(N)
    }
}

// Not taking ownership
impl<const N: usize> From<&CandleCudaDevice<N>> for crate::backend::candle::CandleDevice {
    fn from(_: &CandleCudaDevice<N>) -> Self {
        Self::Cuda(N)
    }
}

impl From<&CandleCpuDevice> for crate::backend::candle::CandleDevice {
    fn from(_: &CandleCpuDevice) -> Self {
        Self::Cpu
    }
}

impl<const N: usize> From<&CandleMetalDevice<N>> for crate::backend::candle::CandleDevice {
    fn from(_: &CandleMetalDevice<N>) -> Self {
        Self::Metal(N)
    }
}
