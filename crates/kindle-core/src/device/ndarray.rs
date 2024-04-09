use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NdArrayCpuDevice;
impl<Element> crate::device::KindleDevice<'dv, crate::backend::NdArray<Element>>
    for NdArrayCpuDevice
{
}

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
