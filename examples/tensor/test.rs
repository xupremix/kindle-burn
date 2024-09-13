#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::Tensor;
///# A 4-Dimensional Tensor
pub struct Tensor4<
    Backend,
    Device,
    const DIM_0: usize,
    const DIM_1: usize,
    const DIM_2: usize,
    const DIM_3: usize,
    Kind = kindle_burn::tensor::Float,
> where
    Backend: kindle_burn::tensor::backend::Backend,
    Kind: kindle_burn::tensor::TensorKind<Backend>,
{
    tensor: kindle_burn::tensor::Tensor<Backend, 4, Kind>,
    _device: std::marker::PhantomData<Device>,
}
#[automatically_derived]
impl<
        Backend: ::core::fmt::Debug,
        Device: ::core::fmt::Debug,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind: ::core::fmt::Debug,
    > ::core::fmt::Debug for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Kind: kindle_burn::tensor::TensorKind<Backend>,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Tensor4",
            "tensor",
            &self.tensor,
            "_device",
            &&self._device,
        )
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::AutodiffBackend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    /// Perform the backward pass of the tensor.
    pub fn backward(
        &self,
    ) -> <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients {
        self.tensor.backward()
    }
    pub fn grad(
        &self,
        grads: &<Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients,
    ) -> Option<
        Tensor4<
            <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
            Device,
            DIM_0,
            DIM_1,
            DIM_2,
            DIM_3,
            kindle_burn::tensor::Float,
        >,
    > {
        match self.tensor.grad(grads) {
            Some(tensor) => Some(Tensor4 {
                tensor,
                _device: std::marker::PhantomData,
            }),
            None => None,
        }
    }
    /// Remove the gradient of the tensor.
    pub fn grad_remove(
        &self,
        grads: &mut <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients,
    ) -> Option<
        Tensor4<
            <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
            Device,
            DIM_0,
            DIM_1,
            DIM_2,
            DIM_3,
            kindle_burn::tensor::Float,
        >,
    > {
        match self.tensor.grad_remove(grads) {
            Some(tensor) => Some(Tensor4 {
                tensor,
                _device: std::marker::PhantomData,
            }),
            None => None,
        }
    }
    /// Replace the gradients
    pub fn grad_replace(
        &self,
        grads: &mut <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients,
        grad: Tensor4<
            <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
            Device,
            DIM_0,
            DIM_1,
            DIM_2,
            DIM_3,
            kindle_burn::tensor::Float,
        >,
    ) {
        self.tensor.grad_replace(grads, grad.tensor)
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::AutodiffBackend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicAutodiffOps<Backend>,
{
    /// Inner tensor without the autodiff
    pub fn inner(
        self,
    ) -> Tensor4<
        <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
        Device,
        DIM_0,
        DIM_1,
        DIM_2,
        DIM_3,
        <Kind as kindle_burn::tensor::BasicAutodiffOps<Backend>>::InnerKind,
    > {
        Tensor4 {
            tensor: self.tensor.inner(),
            _device: std::marker::PhantomData,
        }
    }
    /// Convert a tensor to the autodiff backend
    pub fn from_inner(
        inner: Tensor4<
            <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
            Device,
            DIM_0,
            DIM_1,
            DIM_2,
            DIM_3,
            <Kind as kindle_burn::tensor::BasicAutodiffOps<Backend>>::InnerKind,
        >,
    ) -> Self {
        Self {
            tensor: kindle_burn::tensor::Tensor::from_inner(inner.tensor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    /// Returns the shape of the tensor.
    pub fn shape(&self) -> kindle_burn::tensor::Shape<4usize> {
        self.tensor.shape()
    }
    /// Returns the dimensions of the tensor.
    pub fn dims(&self) -> [usize; 4usize] {
        self.tensor.dims()
    }
    /// Returns the device of the tensor.
    pub fn device(&self) -> <Backend as kindle_burn::tensor::backend::Backend>::Device {
        self.tensor.device()
    }
    /// Returns a new tensor on the given device.
    pub fn to_device<NewDevice: kindle_burn::device::KindleDevice<Backend>>(
        self,
    ) -> Tensor4<Backend, NewDevice, DIM_0, DIM_1, DIM_2, DIM_3, Kind> {
        Tensor4 {
            tensor: self.tensor.to_device(&NewDevice::to_device()),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns the data of the tensor
    pub fn into_data(self) -> kindle_burn::tensor::TensorData {
        self.tensor.into_data()
    }
    /// Returns the data of the tensor without taking ownership.
    pub fn to_data(&self) -> kindle_burn::tensor::TensorData {
        self.tensor.to_data()
    }
    /// Creates a new tensor from the given data.
    ///
    /// WARNING: This function panics if the data doesn't match the dimensions provided.
    pub fn from_data_unchecked<TensorData>(data: TensorData) -> Self
    where
        TensorData: Into<kindle_burn::tensor::TensorData>,
    {
        let curr_dims = [DIM_0, DIM_1, DIM_2, DIM_3];
        let data = data.into();
        // for (dim, curr_dim) in data.shape.dims.iter().zip(curr_dims.iter()) {
        for (dim, curr_dim) in data.shape.iter().zip(curr_dims.iter()) {
            match (&dim, &curr_dim) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "Expected dimension {0} but got {1}",
                                curr_dim, dim,
                            )),
                        );
                    }
                }
            };
        }
        Self {
            tensor: kindle_burn::tensor::Tensor::from_data(data, &Device::to_device()),
            _device: std::marker::PhantomData,
        }
    }
    /// Applies element-wise equal comparison and returns the respective boolean tensor.
    pub fn equal(
        self,
        other: Self,
    ) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Bool> {
        Tensor4 {
            tensor: self.tensor.equal(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
    /// Applies element-wise not equal comparison and returns the respective boolean tensor.
    pub fn not_equal(
        self,
        other: Self,
    ) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Bool> {
        Tensor4 {
            tensor: self.tensor.not_equal(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns the scalar value of the tensor.
    ///
    /// WARNING: This function panics if the tensor doesn't contain a value.
    pub fn into_scalar(self) -> <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem {
        self.tensor.into_scalar()
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    /// Transposes the tensor.
    pub fn transpose(self) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_3, DIM_2, Kind> {
        Tensor4 {
            tensor: self.tensor.transpose(),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    /// Creates an Empty tensor.
    pub fn empty() -> Self {
        Self {
            tensor: kindle_burn::tensor::Tensor::empty(
                [DIM_0, DIM_1, DIM_2, DIM_3],
                &Device::to_device(),
            ),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    /// Returns the tensor containing the elements selected from the ranges
    pub fn slice<
        const RANGE_0_0: usize,
        const RANGE_0_1: usize,
        const RANGE_1_0: usize,
        const RANGE_1_1: usize,
        const RANGE_2_0: usize,
        const RANGE_2_1: usize,
        const RANGE_3_0: usize,
        const RANGE_3_1: usize,
    >(
        self,
    ) -> Tensor4<Backend, Device, RANGE_0_1, RANGE_1_1, RANGE_2_1, RANGE_3_1, Kind> {
        let range_0 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_0,
            RANGE_0_0,
            RANGE_0_1,
        >>::new();
        let range_1 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_1,
            RANGE_1_0,
            RANGE_1_1,
        >>::new();
        let range_2 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_2,
            RANGE_2_0,
            RANGE_2_1,
        >>::new();
        let range_3 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_3,
            RANGE_3_0,
            RANGE_3_1,
        >>::new();
        Tensor4 {
            tensor: self.tensor.slice([range_0, range_1, range_2, range_3]),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns a copy of the current tensor with the selected
    /// elements changed to the new ones at the selectet indices
    pub fn slice_assign<
        const V_DIM_0: usize,
        const V_DIM_1: usize,
        const V_DIM_2: usize,
        const V_DIM_3: usize,
    >(
        self,
        values: Tensor4<Backend, Device, V_DIM_0, V_DIM_1, V_DIM_2, V_DIM_3, Kind>,
    ) -> Self
    where
        Backend: kindle_burn::tensor::backend::Backend,
        Device: kindle_burn::device::KindleDevice<Backend>,
        Kind: kindle_burn::tensor::BasicOps<Backend>,
    {
        let range_0 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            V_DIM_0,
            0,
            V_DIM_0,
        >>::new();
        let range_1 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            V_DIM_1,
            0,
            V_DIM_1,
        >>::new();
        let range_2 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            V_DIM_2,
            0,
            V_DIM_2,
        >>::new();
        let range_3 = <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            V_DIM_3,
            0,
            V_DIM_3,
        >>::new();
        Self {
            tensor: self
                .tensor
                .slice_assign([range_0, range_1, range_2, range_3], values.tensor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::Swap<0usize, 1usize>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_1, DIM_0, DIM_2, DIM_3, Kind>;
    ///Swaps dimensions 0 and 1 of the tensor.
    fn swap_dims(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.swap_dims(0usize, 1usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::Swap<0usize, 2usize>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_2, DIM_1, DIM_0, DIM_3, Kind>;
    ///Swaps dimensions 0 and 2 of the tensor.
    fn swap_dims(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.swap_dims(0usize, 2usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::Swap<0usize, 3usize>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_3, DIM_1, DIM_2, DIM_0, Kind>;
    ///Swaps dimensions 0 and 3 of the tensor.
    fn swap_dims(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.swap_dims(0usize, 3usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::Swap<1usize, 2usize>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_2, DIM_1, DIM_3, Kind>;
    ///Swaps dimensions 1 and 2 of the tensor.
    fn swap_dims(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.swap_dims(1usize, 2usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::Swap<1usize, 3usize>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_3, DIM_2, DIM_1, Kind>;
    ///Swaps dimensions 1 and 3 of the tensor.
    fn swap_dims(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.swap_dims(1usize, 3usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::Swap<2usize, 3usize>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, DIM_3, DIM_2, Kind>;
    ///Swaps dimensions 2 and 3 of the tensor.
    fn swap_dims(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.swap_dims(2usize, 3usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    /// Concatenates the tensors along the given dimension.
    /// WARNING: This function panics if the provided dimension results are not == to the
    /// resulting sum of the dimensions of the tensors.
    fn cat_unchecked<const DIM: usize>(
        tensors: &[kindle_burn::tensor::Tensor<Backend, 4usize, Kind>],
    ) -> Self {
        <kindle_burn::const_assert::Value as kindle_burn::const_assert::ConstValueBetween<
            DIM,
            0,
            4usize,
        >>::VALID;
        let ty_dim = [DIM_0, DIM_1, DIM_2, DIM_3];
        let dim = ty_dim[DIM];
        let sum = tensors.iter().map(|t| t.dims()[DIM]).sum::<usize>();
        match (&sum, &dim) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(format_args!(
                            "Wrong dimension sum provided, expected: {0}, got: {1}",
                            sum, dim,
                        )),
                    );
                }
            }
        };
        for i in 0..4usize {
            if i == DIM {
                continue;
            }
            if tensors.iter().any(|t| t.dims()[i] != ty_dim[i]) {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "The other dimensions provided are not equal"
                    ));
                };
            }
        }
        Self {
            tensor: kindle_burn::tensor::Tensor::<Backend, 4usize, Kind>::cat(
                tensors.iter().cloned().collect::<Vec<_>>(),
                DIM,
            ),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
        const START: usize,
        const LENGTH: usize,
    > kindle_burn::dimensions::Narrow<0usize, START, LENGTH>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, LENGTH, DIM_1, DIM_2, DIM_3, Kind>;
    ///Narrows the tensor along dimension 0
    fn narrow(self) -> Self::Output {
        <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_0,
            START,
            LENGTH,
        >>::check();
        Tensor4 {
            tensor: self.tensor.narrow(0usize, START, LENGTH),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
        const START: usize,
        const LENGTH: usize,
    > kindle_burn::dimensions::Narrow<1usize, START, LENGTH>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, LENGTH, DIM_2, DIM_3, Kind>;
    ///Narrows the tensor along dimension 1
    fn narrow(self) -> Self::Output {
        <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_1,
            START,
            LENGTH,
        >>::check();
        Tensor4 {
            tensor: self.tensor.narrow(1usize, START, LENGTH),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
        const START: usize,
        const LENGTH: usize,
    > kindle_burn::dimensions::Narrow<2usize, START, LENGTH>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, LENGTH, DIM_3, Kind>;
    ///Narrows the tensor along dimension 2
    fn narrow(self) -> Self::Output {
        <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_2,
            START,
            LENGTH,
        >>::check();
        Tensor4 {
            tensor: self.tensor.narrow(2usize, START, LENGTH),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
        const START: usize,
        const LENGTH: usize,
    > kindle_burn::dimensions::Narrow<3usize, START, LENGTH>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, LENGTH, Kind>;
    ///Narrows the tensor along dimension 3
    fn narrow(self) -> Self::Output {
        <kindle_burn::const_assert::Range as kindle_burn::const_assert::ConstRange<
            0,
            DIM_3,
            START,
            LENGTH,
        >>::check();
        Tensor4 {
            tensor: self.tensor.narrow(3usize, START, LENGTH),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Bool>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    /// Creates a boolean tensor from the given data
    /// WARNING: panics if the dimensions provided do not match
    pub fn from_bool_unchecked<TensorData>(data: TensorData) -> Self
    where
        TensorData: Into<kindle_burn::tensor::TensorData>,
    {
        let curr_dims = [DIM_0, DIM_1, DIM_2, DIM_3];
        let data = data.into();
        // for (dim, curr_dim) in data.shape.dims.iter().zip(curr_dims.iter()) {
        for (dim, curr_dim) in data.shape.iter().zip(curr_dims.iter()) {
            match (&dim, &curr_dim) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "Expected dimension {0} but got {1}",
                                curr_dim, dim,
                            )),
                        );
                    }
                }
            };
        }
        Self {
            tensor:
                kindle_burn::tensor::Tensor::<Backend, 4usize, kindle_burn::tensor::Bool>::from_bool(
                    data,
                    &Device::to_device(),
                ),
            _device: std::marker::PhantomData,
        }
    }
    /// Convert to a int tensor
    pub fn int(
        self,
    ) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Int> {
        Tensor4 {
            tensor: self.tensor.int(),
            _device: std::marker::PhantomData,
        }
    }
    /// Convert to a float tensor
    pub fn float(
        self,
    ) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float> {
        Tensor4 {
            tensor: self.tensor.float(),
            _device: std::marker::PhantomData,
        }
    }
    /// Invert the boolean tensor
    pub fn bool_not(self) -> Self {
        Self {
            tensor: self.tensor.bool_not(),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    /// Perform in implace operation on the underlying tensor
    pub fn inplace<F>(&mut self, f: F)
    where
        F: FnOnce(
            kindle_burn::tensor::Tensor<Backend, 4usize, kindle_burn::tensor::Float>,
        )
            -> kindle_burn::tensor::Tensor<Backend, 4usize, kindle_burn::tensor::Float>,
    {
        self.tensor.inplace(f);
    }
    /// Perform the exp fn (e^x)
    pub fn exp(self) -> Self {
        Self {
            tensor: self.tensor.exp(),
            _device: std::marker::PhantomData,
        }
    }
    /// Perform the log fn (ln(x))
    pub fn log(self) -> Self {
        Self {
            tensor: self.tensor.log(),
            _device: std::marker::PhantomData,
        }
    }
    /// Perform the log op (log(x + 1))
    pub fn log1p(self) -> Self {
        Self {
            tensor: self.tensor.log1p(),
            _device: std::marker::PhantomData,
        }
    }
    /// Apply the error function
    pub fn erf(self) -> Self {
        Self {
            tensor: self.tensor.erf(),
            _device: std::marker::PhantomData,
        }
    }
    /// Apply the element-wise reciprocal
    pub fn recip(self) -> Self {
        Self {
            tensor: self.tensor.recip(),
            _device: std::marker::PhantomData,
        }
    }
    /// Apply the element-wise sin function
    pub fn sin(self) -> Self {
        Self {
            tensor: self.tensor.sin(),
            _device: std::marker::PhantomData,
        }
    }
    /// Apply the element-wise cos function
    pub fn cos(self) -> Self {
        Self {
            tensor: self.tensor.cos(),
            _device: std::marker::PhantomData,
        }
    }
    /// Apply the element-wise tanh function
    pub fn tanh(self) -> Self {
        Self {
            tensor: self.tensor.tanh(),
            _device: std::marker::PhantomData,
        }
    }
    /// Apply the element-wise square root function
    pub fn sqrt(self) -> Self {
        Self {
            tensor: self.tensor.sqrt(),
            _device: std::marker::PhantomData,
        }
    }
    /// Creates a float tensor from the given data
    /// WARNING: panics if the dimensions provided do not match
    pub fn from_float_unchecked<D>(data: D) -> Self
    where
        D: Into<kindle_burn::tensor::TensorData>,
    {
        let curr_dims = [DIM_0, DIM_1, DIM_2, DIM_3];
        let data = data.into();
        // for (dim, curr_dim) in data.shape.dims.iter().zip(curr_dims.iter()) {
        for (dim, curr_dim) in data.shape.iter().zip(curr_dims.iter()) {
            match (&dim, &curr_dim) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "Expected dimension {0} but got {1}",
                                curr_dim, dim,
                            )),
                        );
                    }
                }
            };
        }
        Self {
            tensor: kindle_burn::tensor::Tensor::<
                Backend,
                4usize,
                kindle_burn::tensor::Float,
            >::from_floats(data, &Device::to_device()),
            _device: std::marker::PhantomData,
        }
    }
    /// Convert to an int tensor
    pub fn int(
        self,
    ) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Int> {
        Tensor4 {
            tensor: self.tensor.int(),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns a zeroed tensor with the same dimensions
    pub fn zeros_like(&self) -> Self {
        Self {
            tensor: self.tensor.zeros_like(),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns a ones tensor with the same dimensions
    pub fn ones_like(&self) -> Self {
        Self {
            tensor: self.tensor.ones_like(),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns a tensor with the same dimensions with values
    /// sampled from the given distribution
    pub fn random_like(&self, distribution: kindle_burn::tensor::Distribution) -> Self {
        Self {
            tensor: self.tensor.random_like(distribution),
            _device: std::marker::PhantomData,
        }
    }
    /// Return the tensor in a full precision backend
    pub fn into_full_precision(
        self,
    ) -> Tensor4<
        <Backend::FullPrecisionBridge as kindle_burn::tensor::backend::BackendBridge<
            Backend,
        >>::Target,
        Device,
        DIM_0,
        DIM_1,
        DIM_2,
        DIM_3,
        kindle_burn::tensor::Float,
    >{
        Tensor4 {
            tensor: self.tensor.into_full_precision(),
            _device: std::marker::PhantomData,
        }
    }
    /// Removes the tensor from the autodiff graph
    pub fn detach(self) -> Self {
        Self {
            tensor: self.tensor.detach(),
            _device: std::marker::PhantomData,
        }
    }
    /// Marks the tensor as requiring gradient tracking
    pub fn require_grad(self) -> Self {
        Self {
            tensor: self.tensor.require_grad(),
            _device: std::marker::PhantomData,
        }
    }
    /// Mark as untracked or tracked
    pub fn set_require_grad(self, require_grad: bool) -> Self {
        Self {
            tensor: self.tensor.set_require_grad(require_grad),
            _device: std::marker::PhantomData,
        }
    }
}
impl<Backend, Device, const CLASSES: usize>
    Tensor4<Backend, Device, 1usize, 1usize, 1usize, CLASSES, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    /// Creates a one hot encoded tensor at the given index
    pub fn one_hot<const IDX: usize>() -> Self {
        _ = <kindle_burn::const_assert::Value as kindle_burn::const_assert::ConstValueBetween<
            IDX,
            0,
            CLASSES,
        >>::VALID;
        Self {
            tensor:
                kindle_burn::tensor::Tensor::<Backend, 4usize, kindle_burn::tensor::Float>::one_hot(
                    IDX,
                    CLASSES,
                    &Device::to_device(),
                ),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    /// Performs matrix multiplication
    /// NOTE: (follows the pytorch - numpy dimension convention)
    /// Es.
    /// DIMS:  <D0, D1, D2, D3>
    /// OTHER: <D0, D1, D3, D4>
    /// RIS:   <D0, D1, D2, D4>
    pub fn matmul<const I_DIM_4: usize>(
        self,
        other: Tensor4<Backend, Device, DIM_0, DIM_1, DIM_3, I_DIM_4, kindle_burn::tensor::Float>,
    ) -> Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, I_DIM_4, kindle_burn::tensor::Float> {
        Tensor4 {
            tensor: self.tensor.matmul(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Covariance<{ 0usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, DIM_3, DIM_3, kindle_burn::tensor::Float>;
    fn cov(self, correction_factor: usize) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.cov(0usize, correction_factor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Covariance<{ 1usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_1, DIM_0, DIM_3, DIM_3, kindle_burn::tensor::Float>;
    fn cov(self, correction_factor: usize) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.cov(1usize, correction_factor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Covariance<{ 2usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_2, DIM_1, DIM_3, DIM_3, kindle_burn::tensor::Float>;
    fn cov(self, correction_factor: usize) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.cov(2usize, correction_factor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Covariance<{ 3usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_3, DIM_1, DIM_0, DIM_0, kindle_burn::tensor::Float>;
    fn cov(self, correction_factor: usize) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.cov(3usize, correction_factor),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AnyDim<{ 0usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, 1usize, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Bool>;
    fn any_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.any_dim(0usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AnyDim<{ 1usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, 1usize, DIM_2, DIM_3, kindle_burn::tensor::Bool>;
    fn any_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.any_dim(1usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AnyDim<{ 2usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, 1usize, DIM_3, kindle_burn::tensor::Bool>;
    fn any_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.any_dim(2usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AnyDim<{ 3usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, 1usize, kindle_burn::tensor::Bool>;
    fn any_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.any_dim(3usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AllDim<{ 0usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, 1usize, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Bool>;
    fn all_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.all_dim(0usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AllDim<{ 1usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, 1usize, DIM_2, DIM_3, kindle_burn::tensor::Bool>;
    fn all_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.all_dim(1usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AllDim<{ 2usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, 1usize, DIM_3, kindle_burn::tensor::Bool>;
    fn all_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.all_dim(2usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > kindle_burn::dimensions::AllDim<{ 3usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::BasicOps<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, 1usize, kindle_burn::tensor::Bool>;
    fn all_dim(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.all_dim(3usize),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Variance<{ 0usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, 1usize, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>;
    fn var(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.var(0usize),
            _device: std::marker::PhantomData,
        }
    }
    fn var_mean(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean(0usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
    fn var_mean_bias(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean_bias(0usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Variance<{ 1usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, 1usize, DIM_2, DIM_3, kindle_burn::tensor::Float>;
    fn var(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.var(1usize),
            _device: std::marker::PhantomData,
        }
    }
    fn var_mean(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean(1usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
    fn var_mean_bias(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean_bias(1usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Variance<{ 2usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, 1usize, DIM_3, kindle_burn::tensor::Float>;
    fn var(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.var(2usize),
            _device: std::marker::PhantomData,
        }
    }
    fn var_mean(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean(2usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
    fn var_mean_bias(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean_bias(2usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
    > kindle_burn::dimensions::Variance<{ 3usize }>
    for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, kindle_burn::tensor::Float>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
{
    type Output = Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, 1usize, kindle_burn::tensor::Float>;
    fn var(self) -> Self::Output {
        Tensor4 {
            tensor: self.tensor.var(3usize),
            _device: std::marker::PhantomData,
        }
    }
    fn var_mean(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean(3usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
    fn var_mean_bias(self) -> (Self::Output, Self::Output) {
        let (var, mean) = self.tensor.var_mean_bias(3usize);
        (
            Tensor4 {
                tensor: var,
                _device: std::marker::PhantomData,
            },
            Tensor4 {
                tensor: mean,
                _device: std::marker::PhantomData,
            },
        )
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::Numeric<Backend>,
    <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem: kindle_burn::tensor::Element,
{
    /// Adds another tensor to the current one
    pub fn add(self, other: Self) -> Self {
        Self {
            tensor: self.tensor.add(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
    /// Adds a value which can be interpreted as a scalar to the tensor
    pub fn add_scalar<Scalar>(self, other: Scalar) -> Self
    where
        Scalar: kindle_burn::prelude::ElementConversion,
    {
        Self {
            tensor: self.tensor.add_scalar(other),
            _device: std::marker::PhantomData,
        }
    }
    /// Subtracts another tensor to the current one
    pub fn sub(self, other: Self) -> Self {
        Self {
            tensor: self.tensor.sub(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
    /// Subtracts a value which can be interpreted as a scalar to the tensor
    pub fn sub_scalar<Scalar>(self, other: Scalar) -> Self
    where
        Scalar: kindle_burn::prelude::ElementConversion,
    {
        Self {
            tensor: self.tensor.sub_scalar(other),
            _device: std::marker::PhantomData,
        }
    }
    /// Divides another tensor with the current one
    pub fn div(self, other: Self) -> Self {
        Self {
            tensor: self.tensor.div(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
    /// Divides the current tensor with a value which can be interpreted as a scalar
    pub fn div_scalar<Scalar>(self, other: Scalar) -> Self
    where
        Scalar: kindle_burn::prelude::ElementConversion,
    {
        Self {
            tensor: self.tensor.div_scalar(other),
            _device: std::marker::PhantomData,
        }
    }
    /// Performs element-wise multiplication with another tensor
    pub fn mul(self, other: Self) -> Self {
        Self {
            tensor: self.tensor.mul(other.tensor),
            _device: std::marker::PhantomData,
        }
    }
    /// Performs element-wise multiplication with another value which can be
    /// interpreted as a Scalar data type
    pub fn mul_scalar<Scalar>(self, other: Scalar) -> Self
    where
        Scalar: kindle_burn::prelude::ElementConversion,
    {
        Self {
            tensor: self.tensor.mul_scalar(other),
            _device: std::marker::PhantomData,
        }
    }
    /// Switches the sign of each element in the current tensor
    pub fn neg(self) -> Self {
        Self {
            tensor: self.tensor.neg(),
            _device: std::marker::PhantomData,
        }
    }
    /// Returns the signs of each element
    pub fn sign(self) -> Self {
        Self {
            tensor: self.tensor.sign(),
            _device: std::marker::PhantomData,
        }
    }
    /// Creates a tensor full of ones
    pub fn ones() -> Self {
        let shape = [DIM_0, DIM_1, DIM_2, DIM_3];
        Self {
            tensor: kindle_burn::tensor::Tensor::<Backend, 4usize, Kind>::ones(
                shape,
                &Device::to_device(),
            ),
            _device: std::marker::PhantomData,
        }
    }
    /// Creates a tensor full of zeros
    pub fn zeros() -> Self {
        let shape = [DIM_0, DIM_1, DIM_2, DIM_3];
        Self {
            tensor: kindle_burn::tensor::Tensor::<Backend, 4usize, Kind>::zeros(
                shape,
                &Device::to_device(),
            ),
            _device: std::marker::PhantomData,
        }
    }
}
impl<
        Backend,
        Device,
        const DIM_0: usize,
        const DIM_1: usize,
        const DIM_2: usize,
        const DIM_3: usize,
        Kind,
    > Clone for Tensor4<Backend, Device, DIM_0, DIM_1, DIM_2, DIM_3, Kind>
where
    Backend: kindle_burn::tensor::backend::Backend,
    Device: kindle_burn::device::KindleDevice<Backend>,
    Kind: kindle_burn::tensor::TensorKind<Backend>,
{
    fn clone(&self) -> Self {
        Self {
            tensor: self.tensor.clone(),
            _device: std::marker::PhantomData,
        }
    }
}
fn main() {
    let t: Tensor<Wgpu, 3> = Tensor::ones([1, 2, 3], &Default::default());
    let t = t.repeat(&[1, 2, 3]);
    {
        ::std::io::_print(format_args!(
            "Shape: {0:?}\nData: {1:?}\n",
            t.shape(),
            t.to_data()
        ));
    };
}
