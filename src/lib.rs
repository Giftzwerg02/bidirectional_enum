macro_rules! bi_enum {
    ($(#[$derives:meta])* $vis:vis enum $name:ident <=> $type:tt {$($(#[$field_derives:meta])* $field_name:ident <=> $val:expr )*}) => {
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
            type Error = ();

            fn try_from(try_from: char) -> Result<Self, Self::Error> {
                match try_from {
                    $($val => Ok($name::$field_name),)*
                    _ => Err(())
                }
            }
        } 
    };
}

bi_enum! {
    #[derive(Debug)]
    enum A <=> char 
    {
        T1 <=> 'a'
        T2 <=> 'b'
    }
}