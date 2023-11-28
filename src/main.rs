/// Notes:
/// - https://stackoverflow.com/questions/40776020/is-there-any-way-to-restrict-a-generic-type-to-one-of-several-types
/// - a new TriVec shall
///     - hold no memory (neither in stack nor in memory)
///     - its length of the data structure should be zero
///
fn main() {
    // let v: TriVec<String> = TriVec::new(20, 1000);
    // assert_eq!(v.capacity(), 0);
}

enum TriVecState {
    Inline,
    Heap,
    Disk,
}

struct TriVec<T, const N: usize> {
    inline_capacity: usize,
    heap_capacity: usize,
    state: TriVecState,
    data: TriVecData<T, N>,
}

impl<T, const N: usize> TriVec<T, N> {
    fn new(heap_capacity: usize) -> Self {
        Self {
            inline_capacity: 100,
            heap_capacity: 10_000,
            state: TriVecState::Inline,
            data: TriVecData::new(),
        }
    }

    pub fn capacity(&self) -> usize {
        match self.state {
            TriVecState::Inline => self.inline_capacity,
            TriVecState::Heap => self.heap_capacity,
            // TODO: Capacity in disk state can be infinite
            TriVecState::Disk => usize::MAX,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, item: T) {
        // match self.data {
        //     TriVecData::Inline(mut inline) => {
        //         let ptr = inline.as_mut_ptr();
        //     }
        //     _ => todo!(),
        // }
    }
}

/// TODO: Consider a Union instead of Enum
enum TriVecData<T, const N: usize> {
    // TODO: We can't initiate Inline variant using the
    // [T; N] syntax because that requires T to be Copy
    //
    // Inline(([T; N])),
    Inline(()),
    Heap(Vec<T>),
    Disk(()),
}

impl<T, const N: usize> TriVecData<T, N> {
    fn new() -> Self {
        Self::Inline(())
    }

    fn len(&self) -> usize {
        match self {
            _ => todo!(),
        }
    }
}
