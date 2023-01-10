# Bidirectional Enums
A macro to automatically generate a two-way binding between an `enum`-type and any other type.

## Example
```rust
use std::error::Error;

#[macro_use]
extern crate bidirectional_enum;

bi_enum! {
    #[derive(Debug)]
    enum SomeEnum <=> char 
    {
        T1 <=> 'a',
        T2 <=> 'b'
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t1 = SomeEnum::T1;

    // from enum to char
    let a = char::from(t1); 
    dbg!(&a); // 'a'

    // from char back to enum
    let t1 = SomeEnum::try_from(a);
    dbg!(&t1); // Ok(SomeEnum::T1)

    // invalid char
    let err = SomeEnum::try_from('c');
    dbg!(&err); // Err(EnumTryFromErr { from: "char", to: "SomeEnum" })

    Ok(())
}
```