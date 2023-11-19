/// Notes:
/// - https://stackoverflow.com/questions/40776020/is-there-any-way-to-restrict-a-generic-type-to-one-of-several-types
/// - a new TriVecta shall
///     - hold no memory (neither in stack nor in memory)
///     - its length of the data structure should be zero
///
fn main() {
    let len = TriVecta::<i8, 20>::inline_capacity();
    assert_eq!(len, 20);
}

struct TriVecta<T, const N: usize> {
    len: usize,
    data: TriVectaData<T, N>,
}

/// Note: Use of constant generics
/// TriVecta is parameterized with N and N is a constant generic
/// This allows us to construct [T; N]
impl<T: Copy + Default, const N: usize> TriVecta<T, N> {
    fn new() -> Self {
        Self {
            len: N,
            data: TriVectaData::Inline([T::default(); N]),
        }
    }

    fn inline_capacity() -> usize {
        N
    }

    fn push(&mut self, item: T) {
        match self.data {
            TriVectaData::Inline(mut inline) => {
                let ptr = inline.as_mut_ptr();
            }
            _ => todo!(),
        }
    }
}

/// TODO: Consider a Union instead of Enum
enum TriVectaData<T, const N: usize> {
    Inline([T; N]),
    Heap(Vec<T>),
    Disk(()),
}

impl<T, const N: usize> TriVectaData<T, N> {
    fn len(self) -> usize {
        match self {
            TriVectaData::Inline(d) => d.len(),
            _ => todo!(),
        }
    }
}
