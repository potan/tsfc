
extern crate tsfc;
use tsfc::*;

#[test]
fn rw() {
  let f = "testfile";
  let data = [13u8, 17u8];
  {
    let mut w = create(f).unwrap();
    let s = w.write(&data).unwrap();
    assert_eq!(2, s);
  }
  {
    let mut rd = [0u8, 0u8];
    let mut r = open(f).unwrap();
    let s = r.read(&mut rd).unwrap();
    assert_eq!(2, s);
    assert_eq!(rd, data);
  }
  std::fs::remove_file(f).unwrap();
}
