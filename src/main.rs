mod buffer;

use buffer::{Buffer, StaticBuffer};

fn main() {
    let mut buf: StaticBuffer<i32> = StaticBuffer::new();
    buf.add(1).unwrap();
    buf.consume();
}
