#[macro_export]
macro_rules! assert_impl {
    ($trait:path, $type:ty) => {
        const _: fn() = || {
            fn assert_impl<T: $trait>() {}
            assert_impl::<$type>();
        };
    };
}
