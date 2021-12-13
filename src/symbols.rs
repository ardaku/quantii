// From: https://github.com/rust-lang/compiler-builtins/blob/a1f1125992970b905aef23535158e9e804ceeb69/src/math.rs

macro_rules! intrinsics {
    () => ();

    // Right now there's a bunch of architecture-optimized intrinsics in the
    // stock compiler-rt implementation. Not all of these have been ported over
    // to Rust yet so when the `c` feature of this crate is enabled we fall back
    // to the architecture-specific versions which should be more optimized. The
    // purpose of this macro is to easily allow specifying this.
    //
    // The `#[maybe_use_optimized_c_shim]` attribute indicates that this
    // intrinsic may have an optimized C version. In these situations the build
    // script, if the C code is enabled and compiled, will emit a cfg directive
    // to get passed to rustc for our compilation. If that cfg is set we skip
    // the Rust implementation, but if the attribute is not enabled then we
    // compile in the Rust implementation.
    (
        #[maybe_use_optimized_c_shim]
        $(#[$($attr:tt)*])*
        pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => (

        #[cfg($name = "optimized-c")]
        pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
            extern $abi {
                fn $name($($argname: $ty),*) -> $ret;
            }
            unsafe {
                $name($($argname),*)
            }
        }

        #[cfg(not($name = "optimized-c"))]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        intrinsics!($($rest)*);
    );

    // We recognize the `#[aapcs_on_arm]` attribute here and generate the
    // same intrinsic but force it to have the `"aapcs"` calling convention on
    // ARM and `"C"` elsewhere.
    (
        #[aapcs_on_arm]
        $(#[$($attr:tt)*])*
        pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => (
        #[cfg(target_arch = "arm")]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern "aapcs" fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        #[cfg(not(target_arch = "arm"))]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        intrinsics!($($rest)*);
    );

    // Like aapcs above we recognize an attribute for the "unadjusted" abi on
    // win64 for some methods.
    (
        #[unadjusted_on_win64]
        $(#[$($attr:tt)*])*
        pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => (
        #[cfg(all(windows, target_pointer_width = "64"))]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern "unadjusted" fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        #[cfg(not(all(windows, target_pointer_width = "64")))]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        intrinsics!($($rest)*);
    );

    // Some intrinsics on win64 which return a 128-bit integer have an.. unusual
    // calling convention. That's managed here with this "abi hack" which alters
    // the generated symbol's ABI.
    //
    // This will still define a function in this crate with the given name and
    // signature, but the actual symbol for the intrinsic may have a slightly
    // different ABI on win64.
    (
        #[win64_128bit_abi_hack]
        $(#[$($attr:tt)*])*
        pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => (
        #[cfg(all(windows, target_arch = "x86_64"))]
        $(#[$($attr)*])*
        pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
            $($body)*
        }

        #[cfg(all(windows, target_arch = "x86_64"))]
        pub mod $name {
            #[cfg_attr(not(feature = "mangled-names"), no_mangle)]
            pub extern $abi fn $name( $($argname: $ty),* )
                -> ::macros::win64_128bit_abi_hack::U64x2
            {
                let e: $ret = super::$name($($argname),*);
                ::macros::win64_128bit_abi_hack::U64x2::from(e)
            }
        }

        #[cfg(not(all(windows, target_arch = "x86_64")))]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        intrinsics!($($rest)*);
    );

    // A bunch of intrinsics on ARM are aliased in the standard compiler-rt
    // build under `__aeabi_*` aliases, and LLVM will call these instead of the
    // original function. The aliasing here is used to generate these symbols in
    // the object file.
    (
        #[arm_aeabi_alias = $alias:ident]
        $(#[$($attr:tt)*])*
        pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => (
        #[cfg(target_arch = "arm")]
        pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
            $($body)*
        }

        #[cfg(target_arch = "arm")]
        pub mod $name {
            #[cfg_attr(not(feature = "mangled-names"), no_mangle)]
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                super::$name($($argname),*)
            }
        }

        #[cfg(target_arch = "arm")]
        pub mod $alias {
            #[cfg_attr(not(feature = "mangled-names"), no_mangle)]
            pub extern "aapcs" fn $alias( $($argname: $ty),* ) -> $ret {
                super::$name($($argname),*)
            }
        }

        #[cfg(not(target_arch = "arm"))]
        intrinsics! {
            $(#[$($attr)*])*
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                $($body)*
            }
        }

        intrinsics!($($rest)*);
    );

    // This is the final catch-all rule. At this point we generate an
    // intrinsic with a conditional `#[no_mangle]` directive to avoid
    // interfering with duplicate symbols and whatnot during testing.
    //
    // The implementation is placed in a separate module, to take advantage
    // of the fact that rustc partitions functions into code generation
    // units based on module they are defined in. As a result we will have
    // a separate object file for each intrinsic. For further details see
    // corresponding PR in rustc https://github.com/rust-lang/rust/pull/70846
    //
    // After the intrinsic is defined we just continue with the rest of the
    // input we were given.
    (
        $(#[$($attr:tt)*])*
        pub extern $abi:tt fn $name:ident( $($argname:ident:  $ty:ty),* ) -> $ret:ty {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => (
        $(#[$($attr)*])*
        pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
            $($body)*
        }

        pub mod $name {
            $(#[$($attr)*])*
            #[cfg_attr(not(feature = "mangled-names"), no_mangle)]
            pub extern $abi fn $name( $($argname: $ty),* ) -> $ret {
                super::$name($($argname),*)
            }
        }

        intrinsics!($($rest)*);
    );
}

macro_rules! no_mangle {
    ($(fn $fun:ident($($iid:ident : $ity:ty),+) -> $oty:ty;)+) => {
        intrinsics! {
            $(
                pub extern "C" fn $fun($($iid: $ity),+) -> $oty {
                    libm::$fun($($iid),+)
                }
            )+
        }
    }
}

// Compiler intrinsics - compiler-builtins does not export these for arm if the OS is not set to
// "none", and we are targeting novusk so we have to do it ourselves.
no_mangle! {
    fn fmin(x: f64, y: f64) -> f64;
    fn fminf(x: f32, y: f32) -> f32;
    fn fmax(x: f64, y: f64) -> f64;
    fn fmaxf(x: f32, y: f32) -> f32;
    // `f64 % f64`
    fn fmod(x: f64, y: f64) -> f64;
    // `f32 % f32`
    fn fmodf(x: f32, y: f32) -> f32;
}
