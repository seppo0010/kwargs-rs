#[macro_export]
macro_rules! def {
    ($name: ident (
                   $($pname:ident:$ptype:ty),*
                   ;
                   $($kwname:ident:$kwtype:ty=$kwvalue:expr),*
                   )
     $(-> $ret: ty)* { $($stmts: stmt)*}) => {
        pub mod $name {
            pub const REQUIRED_ARGS:&'static [&'static str] = &[$(stringify!($pname)),*];
            pub const OPTIONAL_ARGS:&'static [&'static str] = &[$(stringify!($kwname)),*];

            pub struct Kwargs {
                $(pub $kwname: $kwtype,)*
            }

            impl Default for Kwargs {
                fn default() -> Self {
                    Kwargs {
                        $(
                            $kwname: $kwvalue,
                        )*
                    }
                }
            }

            pub fn myfunc(
                    $($pname:$ptype),*,
                    kwargs: Kwargs
                    ) $(-> $ret)* {
                $(
                    let $kwname = kwargs.$kwname;
                );*
                $($stmts)*
            }
        }
    }
}

#[macro_export]
macro_rules! call {
    ($name: ident(
                  $($param:expr),*
                   $(;
                   $($kwname:ident=$kwvalue:expr),*
                   )*
                  )) => {{
        let args = $name::Kwargs::default();
        $(
            let mut args = args;
            $(
                args.$kwname = $kwvalue;
            )*
        )*
        $name::myfunc($($param),*, args)
    }}
}
