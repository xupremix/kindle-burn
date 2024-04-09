use serde::{Deserialize, Serialize};

macro_rules! tch_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $device <
            $(const $n: usize,)?
            Element = f32,
        >
        where
            Element: crate::backend::libtorch::TchElement,
        {
            _element: std::marker::PhantomData<Element>,
        }
    };
}

#[cfg(not(feature = "autodiff"))]
macro_rules! impl_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        impl<
            $(const $n: usize,)?
            Element
        >   crate::device::KindleDevice<
                crate::backend::LibTorch<Element>,
            > for $device <$($n,)? Element>
        where
            Element: crate::backend::libtorch::TchElement,
        {
            fn to_device() -> crate::backend::libtorch::LibTorchDevice {
                crate::backend::libtorch::LibTorchDevice::$device_variant $(($n))?
            }
        }
    };
}

tch_device!(LibTorchCudaDevice, Cuda, N);
tch_device!(LibTorchCpuDevice, Cpu);
tch_device!(LibTorchMpsDevice, Mps);
tch_device!(LibTorchVulkanDevice, Vulkan);

#[cfg(not(feature = "autodiff"))]
impl_device!(LibTorchCudaDevice, Cuda, N);
#[cfg(not(feature = "autodiff"))]
impl_device!(LibTorchCpuDevice, Cpu);
#[cfg(not(feature = "autodiff"))]
impl_device!(LibTorchMpsDevice, Mps);
#[cfg(not(feature = "autodiff"))]
impl_device!(LibTorchVulkanDevice, Vulkan);
