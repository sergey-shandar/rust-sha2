extern crate num_traits;

type Result<T> = [T; 8];

type Input<T> = [T; 16];

struct State<T> {
    result: Result<T>,
    w: Vec<T>
}

impl<T> State<T>
    where T : num_traits::PrimInt
{
    fn new(w_size: usize, initial: &Result<T>) -> Self {
        Self {
            result: *initial,
            w: vec![T::zero(); w_size]
        }
    }

    fn add(input: &Input<T>) {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
