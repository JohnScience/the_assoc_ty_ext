#![doc = include_str!("../README.md")]
#![no_std]

/// Extension trait that offers the canonical associated type.
///
/// In [New Type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html#:~:text=The%20newtype%20idiom%20gives%20compile,a%20value%20of%20type%20Years%20.&text=Uncomment%20the%20last%20print%20statement,type%20supplied%20must%20be%20Years%20.)
/// `<NewType as `[`TheAssocTyExt`]`>::TheAssocTy` where `struct NewType(T);` would be the type `T`, if implemented.
/// 
/// # Example:
/// 
/// ```
/// use the_assoc_ty_ext::TheAssocTyExt;
/// use core::{
///     default::Default,
///     mem::{size_of, size_of_val},
///     marker::PhantomData
/// };
/// 
/// #[derive(Default)]
/// struct ZST<T: ?Sized>(PhantomData<*const T>);
/// 
/// impl<T: ?Sized> TheAssocTyExt for ZST<T> {
///     type TheAssocTy = T;
/// }
/// 
/// // Repr is necessary to ensure the size of the discriminant
/// #[repr(u8)]
/// enum PrimUnsignedIntKinds {
///     U8(ZST<u8>),
///     U16(ZST<u16>),
///     U32(ZST<u32>),
///     U64(ZST<u64>),
///     U128(ZST<u128>),
///     Usize(ZST<usize>),
/// }
/// 
/// assert_eq!(size_of::<ZST<u16>>(), 0);
/// assert_eq!(
///     size_of_val(&PrimUnsignedIntKinds::U16(ZST::<u16>::default())),
///     size_of::<u8>()
/// );
/// assert_eq!(
///     size_of::<PrimUnsignedIntKinds>(),
///     size_of::<core::mem::Discriminant<PrimUnsignedIntKinds>>()
/// );
/// ```    
pub trait TheAssocTyExt {
    /// The canonical associated type.
    /// 
    /// In [New Type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html#:~:text=The%20newtype%20idiom%20gives%20compile,a%20value%20of%20type%20Years%20.&text=Uncomment%20the%20last%20print%20statement,type%20supplied%20must%20be%20Years%20.)
    /// `<NewType as `[`TheAssocTyExt`]`>::TheAssocTy` where `struct NewType(T);` would be the type `T`, if implemented.
    /// 
    /// # Example
    /// 
    /// ```
    /// use the_assoc_ty_ext::TheAssocTyExt;
    /// use core::{
    ///     default::Default,
    ///     mem::{size_of, size_of_val},
    ///     marker::PhantomData
    /// };
    /// 
    /// #[derive(Default)]
    /// struct ZST<T: ?Sized>(PhantomData<*const T>);
    /// 
    /// impl<T: ?Sized> TheAssocTyExt for ZST<T> {
    ///     type TheAssocTy = T;
    /// }
    /// 
    /// // Repr is necessary to ensure the size of the discriminant
    /// #[repr(u8)]
    /// enum PrimUnsignedIntKinds {
    ///     U8(ZST<u8>),
    ///     U16(ZST<u16>),
    ///     U32(ZST<u32>),
    ///     U64(ZST<u64>),
    ///     U128(ZST<u128>),
    ///     Usize(ZST<usize>),
    /// }
    /// 
    /// assert_eq!(size_of::<ZST<u16>>(), 0);
    /// assert_eq!(
    ///     size_of_val(&PrimUnsignedIntKinds::U16(ZST::<u16>::default())),
    ///     size_of::<u8>()
    /// );
    /// assert_eq!(
    ///     size_of::<PrimUnsignedIntKinds>(),
    ///     size_of::<core::mem::Discriminant<PrimUnsignedIntKinds>>()
    /// );
    /// ```
    type TheAssocTy: ?Sized;
}
