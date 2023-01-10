use std::{error::Error, fmt};

#[derive(Debug)]
pub struct EnumTryFromError {
    from: String,
    to: String
}

impl EnumTryFromError {
    pub fn new(from: String, to: String) -> Self {
        EnumTryFromError { from, to }
    }
}

impl fmt::Display for EnumTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to convert `{}` into `{}`", self.from, self.to)
    }
}

impl Error for EnumTryFromError {}

pub mod bi_enum {
    #[macro_export]
    macro_rules! bi_enum {    
        ($(#[$derives:meta])* $vis:vis enum $name:ident <=> $type:tt {$($(#[$field_derives:meta])* $field_name:ident <=> $val:expr),*}) => {
            $(#[$derives])*
            $vis enum $name {
                $(
                    $(#[$field_derives])* 
                    $field_name,
                )*
            }
    
            impl From<$name> for $type {
                fn from(from: $name) -> Self {
                    match from {
                        $($name::$field_name => $val,)*
                    }
                }
            } 
    
            impl TryFrom<$type> for $name {
                type Error = $crate::EnumTryFromError;
    
                fn try_from(try_from: $type) -> Result<Self, Self::Error> {
                    match try_from {
                        $($val => Ok($name::$field_name),)*
                        _ => Err($crate::EnumTryFromError::new(stringify!($type).to_string(), stringify!($name).to_string()))
                    }
                }
            } 
        };
    }
}