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
    path_error => PathError = 1,
    file_error  => FileError  = 2,
    match_error  => MatchError  = 3,
}
