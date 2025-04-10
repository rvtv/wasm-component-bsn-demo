// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod summit25 {
    pub mod validator {
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod string_length_validation {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn validate(value: &str, length: i32) -> bool {
                unsafe {
                    let vec0 = value;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "summit25:validator/string-length-validation@0.0.1"
                    )]
                    unsafe extern "C" {
                        #[link_name = "validate"]
                        fn wit_import1(_: *mut u8, _: usize, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    unsafe extern "C" fn wit_import1(
                        _: *mut u8,
                        _: usize,
                        _: i32,
                    ) -> i32 {
                        unreachable!()
                    }
                    let ret = unsafe {
                        wit_import1(ptr0.cast_mut(), len0, _rt::as_i32(&length))
                    };
                    _rt::bool_lift(ret as u8)
                }
            }
        }
    }
}
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod summit25 {
        pub mod validator {
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod string_validation {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_validate_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::validate(_rt::string_lift(bytes0));
                    match result1 {
                        true => 1,
                        false => 0,
                    }
                }
                pub trait Guest {
                    fn validate(value: _rt::String) -> bool;
                }
                #[doc(hidden)]
                macro_rules! __export_summit25_validator_string_validation_0_0_1_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "summit25:validator/string-validation@0.0.1#validate")] unsafe
                        extern "C" fn export_validate(arg0 : * mut u8, arg1 : usize,) ->
                        i32 { unsafe { $($path_to_types)*:: _export_validate_cabi::<$ty >
                        (arg0, arg1) } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_summit25_validator_string_validation_0_0_1_cabi;
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_nineproof_validator_ext_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::summit25::validator::string_validation::__export_summit25_validator_string_validation_0_0_1_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::summit25::validator::string_validation);
    };
}
#[doc(inline)]
pub(crate) use __export_nineproof_validator_ext_impl as export;
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:summit25:validator@0.0.1:nineproof-validator-ext:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 366] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe0\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x02\x05values\x06lengthz\0\x7f\x04\0\x08validate\x01\0\x03\0\
1summit25:validator/string-length-validation@0.0.1\x05\0\x01B\x02\x01@\x01\x05va\
lues\0\x7f\x04\0\x08validate\x01\0\x04\0*summit25:validator/string-validation@0.\
0.1\x05\x01\x04\00summit25:validator/nineproof-validator-ext@0.0.1\x04\0\x0b\x1d\
\x01\0\x17nineproof-validator-ext\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.227.1\x10wit-bindgen-rust\x060.41.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
