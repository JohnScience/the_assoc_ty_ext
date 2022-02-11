# Canonical associated type (as in New Type idiom)

One good way to represent types themselves as opposed to the values thereof is to use [zero-sized types][ZST], which are commonly abbreviated as **ZST**s. A programmer can have an enum whose variants store the corresponding [ZST] (and, implicitly, the [enum discriminant] that may or may not be optimized out); and then the size of the enum will not depend on the size of the types that are being represented.

At the time of writing, there is no canonical wrapper that would allow one to get [ZST] representing a type. The closest alternative is [`core::marker::PhantomData`][`PhantomData`], yet the semantics of [`PhantomData`] is [**unnecessarily richer**][`PhantomData<*const T>`] and has undesirable tradeoffs between generality/ease-of-use, **for this, secondary use case**.

**Note**: Even more precisely, the desired `ZST<T>` generic [ZST] [New Type] is close to `PhantomData<*const T>`.

Since `ZST<T>` must represent `T`, it is natural to desire accessing **the** associated type. This crate offers `TheAssocTyExt` trait that is meant to be implemented for any type that has the associated type that is intended to be accessed.

# Example

```rust
use the_assoc_ty_ext::TheAssocTyExt;
use core::{
    default::Default,
    mem::{size_of, size_of_val},
    marker::PhantomData
};

#[derive(Default)]
struct ZST<T: ?Sized>(PhantomData<*const T>);

impl<T: ?Sized> TheAssocTyExt for ZST<T> {
    type TheAssocTy = T;
}

// Repr is necessary to ensure the size of the discriminant
#[repr(u8)]
enum PrimUnsignedIntKinds {
    U8(ZST<u8>),
    U16(ZST<u16>),
    U32(ZST<u32>),
    U64(ZST<u64>),
    U128(ZST<u128>),
    Usize(ZST<usize>),
}

assert_eq!(size_of::<ZST<u16>>(), 0);
assert_eq!(
    size_of_val(&PrimUnsignedIntKinds::U16(ZST::<u16>::default())),
    size_of::<u8>()
);
assert_eq!(
    size_of::<PrimUnsignedIntKinds>(),
    size_of::<core::mem::Discriminant<PrimUnsignedIntKinds>>()
);
```

**Note**: [`core::mem::Discriminant`][`Discriminant`], as the doc states, is [opaque]. Similarly, the data layout of any `enum` is rather vaguely specified for performance considerations. You can learn the deailts in [Unsafe Code Guidelines Reference].

[ZST]: https://runrust.miraheze.org/wiki/Zero-sized_type
[enum discriminant]: https://doc.rust-lang.org/beta/unstable-book/language-features/arbitrary-enum-discriminant.html
[`PhantomData`]: https://doc.rust-lang.org/beta/core/marker/struct.PhantomData.html
[`PhantomData<*const T>`]: https://doc.rust-lang.org/beta/core/marker/struct.PhantomData.html#ownership-and-the-drop-check
[New Type]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[`Discriminant`]: https://doc.rust-lang.org/core/mem/struct.Discriminant.html
[opaque]: https://en.wikipedia.org/wiki/Opaque_data_type
[Unsafe Code Guidelines Reference]: https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html