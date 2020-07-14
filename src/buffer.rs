/// Special error returned when client attempts to add to full buffer.
#[derive(Debug)]
pub struct BufferFullError<T>(T);

/// A buffer manages a fixed amount of data.
///
/// The data are produced one at a time or consumed all at once.
pub trait Buffer<T> {
    /// Add the given element to the buffer if possible.
    fn add(&mut self, element: T) -> Result<(), BufferFullError<T>>;

    /// Copy and return all elements of the buffer.
    ///
    /// When this operation completes, the buffer state will reset so that the buffer
    /// has no elements. In other words, the buffer elements are completely consumed
    /// and ownership transferred.
    ///
    /// Note that the underlying memory is *not* zeroed, so sensitive information should
    /// not be stored in this buffer.
    fn consume(&mut self) -> Option<Vec<T>>;

    /// Returns the number of currently elements in the buffer.
    fn used(&self) -> usize;

    /// Returns the maximum number of elements allowed in the buffer.
    fn capacity(&self) -> usize;
}

const BUFFER_SIZE: usize = 512;

/// Buffer whose capacity is determined at compile time.
pub struct StaticBuffer<T>
where
    T: Copy + Default + Sized,
{
    elements: [T; BUFFER_SIZE],
    next: usize,
}

impl<T> StaticBuffer<T>
where
    T: Copy + Default,
{
    /// Constructs a new buffer.
    pub fn new() -> StaticBuffer<T> {
        StaticBuffer {
            elements: [Default::default(); BUFFER_SIZE],
            next: 0,
        }
    }
}

impl<T> Buffer<T> for StaticBuffer<T>
where
    T: Copy + Default,
{
    fn add(&mut self, element: T) -> Result<(), BufferFullError<T>> {
        if self.next == BUFFER_SIZE {
            return Err(BufferFullError(element));
        }
        self.elements[self.next] = element;
        self.next += 1;
        Ok(())
    }

    fn consume(&mut self) -> Option<Vec<T>> {
        if self.next == 0 {
            return None;
        }
        let mut data = vec![Default::default(); self.next];
        data.copy_from_slice(&self.elements[..self.next]);
        self.next = 0;
        Some(data)
    }

    fn used(&self) -> usize {
        self.next
    }

    fn capacity(&self) -> usize {
        BUFFER_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_buffer_empty() {
        let mut buffer: StaticBuffer<i32> = StaticBuffer::new();
        assert_eq!(buffer.used(), 0);
        assert_eq!(buffer.capacity(), BUFFER_SIZE);
        assert!(buffer.consume().is_none());
    }

    #[test]
    fn static_buffer_add_one() {
        let mut buffer: StaticBuffer<usize> = StaticBuffer::new();

        let expected = 123;
        buffer.add(expected).unwrap();
        assert_eq!(buffer.used(), 1);

        let actual = buffer.consume().unwrap();
        assert_eq!(actual, vec![expected]);
    }

    #[test]
    fn static_buffer_add_until_full() {
        let mut buffer: StaticBuffer<usize> = StaticBuffer::new();

        // Add until full, making sure buffer does not error.
        for value in 0..BUFFER_SIZE {
            assert!(buffer.add(value).is_ok());
        }
        assert_eq!(buffer.used(), BUFFER_SIZE);

        // Try to add when full, check added value is returned with error.
        let expected = BUFFER_SIZE;
        if let Err(BufferFullError(actual)) = buffer.add(expected) {
            assert_eq!(actual, expected);
        } else {
            panic!("should have returned error");
        }
        assert_eq!(buffer.used(), BUFFER_SIZE);

        // Now consume everything.
        let actual = buffer.consume().unwrap();
        let expected: Vec<usize> = (0..BUFFER_SIZE).collect();
        assert_eq!(actual, expected);

        // And finally check we can add again.
        let expected = 34;
        assert!(buffer.add(expected).is_ok());
        assert_eq!(buffer.used(), 1);
        let actual = buffer.consume().unwrap();
        assert_eq!(actual, vec![expected]);
    }
}
