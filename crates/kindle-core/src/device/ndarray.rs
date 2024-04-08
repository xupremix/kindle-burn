use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NdArrayCpuDevice;
impl crate::Sealed for NdArrayCpuDevice {}
impl crate::device::KindleDevice<'_> for NdArrayCpuDevice {}

impl From<NdArrayCpuDevice> for crate::backend::ndarray::NdArrayDevice {
    fn from(_: NdArrayCpuDevice) -> Self {
        Self::Cpu
    }
}

impl From<&NdArrayCpuDevice> for crate::backend::ndarray::NdArrayDevice {
    fn from(_: &NdArrayCpuDevice) -> Self {
        Self::Cpu
    }
}
