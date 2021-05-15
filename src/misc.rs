use crate::Value;

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Bool(b)
    }
}

impl From<u8> for Value {
    fn from(n: u8) -> Value {
        Value::U8(n)
    }
}

impl From<u16> for Value {
    fn from(n: u16) -> Value {
        Value::U16(n)
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Value {
        Value::U32(n)
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Value {
        Value::U64(n)
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Value {
        Value::I8(n)
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Value {
        Value::I16(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Value {
        Value::I32(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Value {
        Value::I64(n)
    }
}

impl From<f32> for Value {
    fn from(n: f32) -> Value {
        Value::F32(n)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Value {
        Value::F64(n)
    }
}

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Value {
        Value::Bytes(v.to_owned())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Value {
        Value::Bytes(v)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Value {
        Value::String(s.to_owned())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}

fn eq_i64(value: &Value, other: i64) -> bool {
    value.as_i64().map_or(false, |i| i == other)
}

fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64().map_or(false, |i| i == other)
}

fn eq_f64(value: &Value, other: f64) -> bool {
    value.as_f64().map_or(false, |i| i == other)
}

fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool().map_or(false, |i| i == other)
}

fn eq_str(value: &Value, other: &str) -> bool {
    value.as_str().map_or(false, |i| i == other)
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}

impl<'a> PartialEq<&'a str> for Value {
    fn eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self)
    }
}

impl<'a> PartialEq<Value> for &'a str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, *self)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
    }
}

macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a mut Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize]
    eq_u64[u8 u16 u32 u64 usize]
    eq_f64[f32 f64]
    eq_bool[bool]
}
