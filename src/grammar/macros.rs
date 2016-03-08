#[macro_export]
macro_rules! earley_count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),+) => (1 + earley_count_exprs!($($tail),*));
}

#[macro_export]
macro_rules! earley_production {
    ($name:expr => $($token:tt),*; ($varname:ident: $vartype:ty) $action:block) => {
        {
            #[derive(Debug, Clone, Eq, PartialEq)]
            struct A([Token; earley_count_exprs!($($token),*)]);

            impl $crate::Production<$vartype> for A {
                fn get_name(&self) -> &'static str {
                    $name
                }

                fn get_tokens(&self) -> &[Token] {
                    &self.0
                }

                #[allow(unused_variables)]
                fn perform<'a>(&self, result: &'a [Node<'a, $vartype>]) -> $vartype {
                    let $varname = result;
                    $action
                }
            }

            Box::new(A([$(earley_expand_token!($token)),*]))
        }
    };
}

#[macro_export]
macro_rules! earley_expand_token {
    ([$name:expr]) => {
        $crate::Token::Terminal($name)
    };
    ({$name:expr}) => {
        $crate::Token::NonTerminal($name)
    };
}
