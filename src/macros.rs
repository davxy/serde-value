/// Helper macro to simplify the construction of complex `serde_value::Value`s.
#[macro_export(local_inner_macros)]
macro_rules! value {
    // Hide distracting implementation details from the generated rustdoc.
    ($($value:tt)+) => {
        value_internal!($($value)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! value_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: value_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        value_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        value_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        value_internal!(@array [$($elems,)* value_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        value_internal!(@array [$($elems,)* value_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        value_internal!(@array [$($elems,)* value_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        value_internal!(@array [$($elems,)* value_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        value_internal!(@array [$($elems,)* value_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        value_internal!(@array [$($elems,)* value_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        value_internal!(@array [$($elems,)* value_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        value_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        value_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: value_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        value_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        value_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        value_internal!(@object $object [$($key)+] (value_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        value_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        value_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        value_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        value_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        value_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        value_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: value_internal!($($value)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::Value::Unit
    };

    ([[$( $element:expr ),*]]) => {
        $crate::Value::Bytes({
            std::vec![$( $element),*]
        })
    };

    ([[$( $element:expr ),+ ,]]) => {
        $crate::value!(( $($element),* ))
    };

    (true) => {
        $crate::Value::Bool(true)
    };

    (false) => {
        $crate::Value::Bool(false)
    };

    ([]) => {
        $crate::Value::Seq(value_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Value::Seq(value_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::Value::Map(std::collections::BTreeMap::new())
    };

    ({ $($tt:tt)+ }) => {
        $crate::Value::Map({
            let mut object = std::collections::BTreeMap::new();
            value_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::to_value(&$other).unwrap()
    };
}

// The value_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! value_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! value_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! value_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}


#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn build_unit_macro() {
        let expected = Value::Unit;

        let val = value!(null);

        assert_eq!(val, expected);
    }

    #[test]
    fn build_bool_macro() {
        let expected = Value::Bool(false);

        let val = value!(false);

        assert_eq!(val, expected);
    }

    #[test]
    fn build_u32_macro() {
        let expected = Value::U32(43);

        let val = value!(43_u32);

        assert_eq!(val, expected);
    }

    #[test]
    fn build_f64_macro() {
        let expected = Value::F64(4.2f64);

        let val = value!(4.2f64);

        assert_eq!(val, expected);
    }

    #[test]
    fn build_string_macro() {
        let expected = Value::String("Hello".to_owned());

        let val: Value = value!("Hello");

        assert_eq!(val, expected);
    }

    #[test]
    fn build_bytes_macro() {
        let expected = Value::Bytes(vec![0x01, 0xFF, 0x80]);

        let val = value!([[0x01, 0xFF, 0x80]]);

        assert_eq!(val, expected);
    }

    #[test]
    fn build_seq_macro() {
        let expected = Value::Seq(vec![
            Value::I32(1),
            Value::String("Foo".to_owned()),
            Value::String("Bar".to_owned()),
            Value::U8(5),
            Value::Map(
                vec![
                    (Value::String("k1".to_owned()), Value::Bool(true)),
                    (
                        Value::String("k2".to_owned()),
                        Value::Bytes(vec![0x01, 0xFF, 0x80]),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        ]);

        let val = value!([1, "Foo", "Bar".to_string(), 3_u8 + 2, { "k1": true, "k2": [[ 0x01, 0xFF, 0x80 ]] }]);

        assert_eq!(val, expected);
    }

    #[test]
    fn build_map_macro() {
        let expected = Value::Map(
            vec![
                (Value::String("k1".to_owned()), Value::I32(123)),
                (
                    Value::String("k2".to_owned()),
                    Value::String("Hello".to_owned()),
                ),
                (
                    Value::String("k3".to_owned()),
                    Value::String("World".to_owned()),
                ),
                (Value::F64(0.01), Value::Char('z')),
                (Value::Bool(false), Value::Unit),
                (Value::I32(1), Value::String("Foo".to_owned())),
                (
                    Value::I32(2),
                    Value::Map(
                        vec![
                            (Value::String("x".to_owned()), Value::I32(456)),
                            (Value::String("y".to_owned()), Value::Bool(true)),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let val = value!({
            "k1": 123,
            "k2": "Hello",
            "k3": "World".to_string(),
            0.01_f64: 'z',
            false: null,
            1: "Foo",
            2: {
                "x": 456,
                "y": true
            }
        });

        assert_eq!(val, expected);
    }
}
