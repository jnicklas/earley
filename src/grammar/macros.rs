#[macro_export]
macro_rules! earley_count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),+) => (1 + earley_count_exprs!($($tail),*));
}

#[macro_export]
macro_rules! earley_production {
    ($name:expr => [$($token:expr),*] ($($varname:pat),*) -> $vartype:ty; $action:block) => {
        {
            #[derive(Debug, Clone, Eq, PartialEq)]
            struct A([$crate::Token; earley_count_exprs!($($token),*)]);

            impl $crate::Production<$vartype> for A {
                fn get_name(&self) -> &'static str {
                    $name
                }

                fn get_tokens(&self) -> &[$crate::Token] {
                    &self.0
                }

                #[allow(unused_variables, unused_mut)]
                fn perform<'a>(&self, result: Vec<$crate::Value<'a, $vartype>>) -> $vartype {
                    println!("{:?}", result);
                    let mut iterator = result.into_iter();
                    $(
                        let $varname = iterator.next().expect("must perform action with same number of arguments as tokens");
                    )*
                    $action
                }
            }

            Box::new(A([$($token),*]))
        }
    };
}
