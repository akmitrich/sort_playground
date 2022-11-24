use rand::Rng;

#[derive(Debug, Default)]
pub struct SortPlayground {
    data: Vec<i64>,
    cmp: usize,
    asg: usize,
}

impl SortPlayground {
    pub fn random(n: usize) -> Self {
        let mut a = vec![Default::default(); n];
        rand::thread_rng().fill(&mut a[..]);
        Self {
            data: a,
            ..Default::default()
        }
    }

    pub fn reversed(n: usize) -> Self {
        let a: Vec<i64> = (0..n).rev().map(|x| x as i64).collect();
        Self { data: a, ..Default::default() }
    }

    pub fn bubble_sort(&mut self) {
        for j in (0..self.data.len()).rev() {
            for i in 0..j {
                self.cmp += 1;
                if self.data[i] > self.data[i + 1] {
                    self.swap(i, i + 1);
                }
            }
        }
    }

    pub fn bubble_opt(&mut self) {
        for j in (0..self.data.len()).rev() {
            let mut may_stop = true;
            for i in 0..j {
                self.cmp += 1;
                if self.data[i] > self.data[i + 1] {
                    may_stop = false;
                    self.swap(i, i + 1);
                }
            }
            if may_stop {
                return;
            }
        }
    }

    pub fn insertion(&mut self) {
        for j in 1..self.data.len() {
            for i in (0..j).rev() {
                self.cmp += 1;
                if self.data[i] <= self.data[i + 1] { break; }
                self.swap(i, i + 1);
            }
        }
    }

    pub fn insertion_shift(&mut self) {
        for j in 1..self.data.len() {
            let item = self.data[j];
            let mut found_index = 0;
            for i in (0..j).rev() {
                self.cmp += 1;
                if self.data[i] > item {
                    self.asg += 1;
                    self.data[i+1] = self.data[i];
                } else {
                    found_index = i + 1;
                    break;
                }
            }
            self.asg += 1;
            self.data[found_index] = item;
        }
    }

    pub fn sorted_percent(&self) -> usize {
        let (all, success, last) = self
            .data
            .iter()
            .skip(1)
            .fold((0, 0, &self.data[0]), |(sum, ok, prev), current| {
                (sum + 1, if prev < current { ok + 1 } else { ok }, current)
            });
        assert_eq!(self.data.last(), Some(last));
        success * 100 / all
    }

    pub fn get_report(&self) -> String {
        format!(
            "Sorted {}% with {} comparisons and {} assignments.",
            self.sorted_percent(),
            self.cmp,
            self.asg
        )
    }

    fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.data.len());
        assert!(b < self.data.len());
        self.data.swap(a, b);
        self.asg += 3; // Suppose we make 3 assignments during swap
    }
}
