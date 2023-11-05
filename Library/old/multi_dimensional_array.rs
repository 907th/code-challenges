// Multi-dimensional array implementation

struct MDArray<T, const N: usize> {
    data: Vec<T>,
    dimensions: [usize; N]
}

impl<T, const N: usize> MDArray<T, N> {
    fn new(dimensions: [usize; N]) -> Self where T: Default + Clone {
        let n = dimensions.iter().product();
        let data = vec![T::default(); n];
        Self { data, dimensions }
    }

    fn get(&self, indices: [usize; N]) -> &T {
        let pos = self.get_data_position_by_indices(&indices);
        &self.data[pos]
    }

    fn set(&mut self, indices: [usize; N], value: T) {
        let pos = self.get_data_position_by_indices(&indices);
        self.data[pos] = value;
    }

    fn get_data_position_by_indices(&self, indices: &[usize; N]) -> usize {
        let mut pos: usize = 0;
        let mut shift: usize = 1;
        for i in (0..N).rev() {
            assert!(indices[i] < self.dimensions[i], "MDArray index {} is out of bound", i);
            pos = pos + indices[i] * shift;
            shift = shift * self.dimensions[i];
        }
        pos
    }
}
