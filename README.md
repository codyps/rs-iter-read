# iter_read

Convert your Iterators over u8 (Iterator<Item=u8>) into Read!

```
extern crate iter_read;

fn main() {
	let mut my_iter = vec![4u8, 6, 2].into_iter();
	let mut my_reader = iter_read::T(my_iter);

	let v = vec![];
	my_reader.read_to_end(&mut v).unwrap();
	println!("{:?}", v);
}
```
