use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct NdArrayCpuDevice<Element = f32>
where
    Element: crate::backend::ndarray::FloatNdArrayElement,
{
    _element: std::marker::PhantomData<Element>,
}

impl<Element> crate::device::KindleDevice<crate::backend::NdArray<Element>>
    for NdArrayCpuDevice<Element>
where
    Element: crate::backend::ndarray::FloatNdArrayElement,
{
    fn to_device() -> crate::backend::ndarray::NdArrayDevice {
        crate::backend::ndarray::NdArrayDevice::Cpu
    }
}

#[cfg(feature = "autodiff")]
impl<Element>
    crate::device::KindleDevice<crate::backend::Autodiff<crate::backend::NdArray<Element>>>
    for NdArrayCpuDevice<Element>
where
    Element: crate::backend::ndarray::FloatNdArrayElement,
{
    fn to_device() -> crate::backend::ndarray::NdArrayDevice {
        crate::backend::ndarray::NdArrayDevice::Cpu
    }
}
