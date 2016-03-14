#[macro_export]
macro_rules! earley_count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),+) => (1 + earley_count_exprs!($($tail),*));
}

#[macro_export]
macro_rules! earley_production {
    ($rule_name_type:ty: $name:expr => [$($token:expr),*] ($($varname:pat),*) -> $output_type:ty; $action:block) => {
        {
            #[derive(Debug, Clone, Eq, PartialEq)]
            struct A([$crate::Token<$rule_name_type>; earley_count_exprs!($($token),*)]);

            impl $crate::Production<$rule_name_type, $output_type> for A {
                fn get_name(&self) -> $rule_name_type {
                    $name
                }

                fn get_tokens(&self) -> &[$crate::Token<$rule_name_type>] {
                    &self.0
                }

                #[allow(unused_variables, unused_mut)]
                fn perform(&self, result: Vec<$crate::Value<$output_type>>) -> $output_type {
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
