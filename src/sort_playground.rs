use rand::Rng;

#[derive(Debug, Default)]
pub struct SortPlayground {
    data: Vec<i64>,
}

impl SortPlayground {
    pub fn random(n: usize) -> Self {
        let mut a = vec![Default::default(); n];
        rand::thread_rng().fill(&mut a[..]);
        Self { data: a }
    }

    pub fn bubble_sort(&mut self) {
        for j in (0..self.data.len()).rev() {
            for i in 0..j {
                if self.data[i] > self.data[i + 1] {
                    self.swap(i, i + 1);
                }
            }
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.data.len());
        assert!(b < self.data.len());
        self.data.swap(a, b)
    }
}