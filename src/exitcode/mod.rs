macro_rules! define_exitcodes {
    ( $( $fn:ident => $variant:ident = $code:expr ),* $(,)? ) => {
        #[repr(i32)]
        pub enum ExitCode {
            $( $variant = $code, )*
        }

        impl ExitCode {
            pub fn code(self) -> i32 { self as i32 }
        }

        $(
            pub fn $fn() -> i32 { ExitCode::$variant.code() }
        )*
    }
}

define_exitcodes! {
    paths_error => WildcardError = 1,
    files_error  => ChangesError  = 2,
}
