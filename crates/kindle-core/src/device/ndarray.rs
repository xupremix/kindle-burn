use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct NdArrayCpuDevice;
impl<'dv, Element> crate::device::KindleDevice<'dv, crate::backend::NdArray<Element>>
    for NdArrayCpuDevice
where
    Element: crate::backend::ndarray::FloatNdArrayElement,
{
    fn to_device() -> crate::backend::ndarray::NdArrayDevice {
        crate::backend::ndarray::NdArrayDevice::Cpu
    }
}
