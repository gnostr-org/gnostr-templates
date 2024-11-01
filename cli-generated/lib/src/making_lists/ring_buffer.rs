pub(crate) fn ring_buffer() {
    println!("ring ring ring");
}

#[derive(Debug)]
struct RingBuffer<T> {
    value: T,
    next: Box<RingBuffer<T>>,
}
