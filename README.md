# tsfc
Typesafe file control in Rust.

This crate do not allow open file for write only and try to read it.

```
extern crate tsfc;
use tsfc::*;

 let mut w = create(f).unwrap();
 w.read(&mut data);
```
and
```
 let mut r = open(f).unwrap();
 r.write(&data);
```
will not be compiled.

