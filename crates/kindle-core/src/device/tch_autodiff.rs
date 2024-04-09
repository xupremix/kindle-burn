macro_rules! impl_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        impl<
            $(const $n: usize,)?
            Element
        >   crate::device::KindleDevice<
            crate::backend::Autodiff<
                crate::backend::LibTorch<Element>,
            >
        > for crate::device::$device <$($n,)? Element>
        where
            Element: crate::backend::libtorch::TchElement,
        {
            fn to_device() -> crate::backend::libtorch::LibTorchDevice {
                crate::backend::libtorch::LibTorchDevice::$device_variant $(($n))?
            }
        }
    };
}

impl_device!(LibTorchCudaDevice, Cuda, N);
impl_device!(LibTorchCpuDevice, Cpu);
impl_device!(LibTorchMpsDevice, Mps);
impl_device!(LibTorchVulkanDevice, Vulkan);
