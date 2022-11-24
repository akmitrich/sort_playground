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
        Self {
            data: a,
            ..Default::default()
        }
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
                if self.data[i] <= self.data[i + 1] {
                    break;
                }
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
                    self.data[i + 1] = self.data[i];
                } else {
                    found_index = i + 1;
                    break;
                }
            }
            self.asg += 1;
            self.data[found_index] = item;
        }
    }

    pub fn insertion_binary(&mut self) {
        for j in 1..self.data.len() {
            let found_index = self.insort_left(self.data[j], 0, j);
            self.perform_insertion(j, found_index);
        }
    }

    pub fn shell_naive(&mut self) {
        let mut gap = self.data.len() / 2;
        while gap > 0 {
            self.perform_shell(gap);
            gap /= 2;
        }
    }

    pub fn shell1(&mut self) {
        const GAP: [usize; 55] = [
            1,
            3,
            7,
            16,
            37,
            83,
            187,
            419,
            937,
            2099,
            4693,
            10499,
            23479,
            52501,
            117391,
            262495,
            586961,
            1312481,
            2934793,
            6562397,
            14673961,
            32811973,
            73369801,
            164059859,
            366848983,
            820299269,
            1834244921,
            4101496331,
            9171224603,
            20507481647,
            45856123009,
            102537408229,
            229280615033,
            512687041133,
            1146403075157,
            2563435205663,
            5732015375783,
            12817176028331,
            28660076878933,
            64085880141667,
            143300384394667,
            320429400708323,
            716501921973329,
            1602147003541613,
            3582509609866643,
            8010735017708063,
            17912548049333207,
            40053675088540303,
            89562740246666023,
            200268375442701509,
            447813701233330109,
            1001341877213507537,
            2239068506166650537,
            5006709386067537661,
            11195342530833252689,
        ];
        for gap in GAP.iter().rev() {
            self.perform_shell(*gap);
        }
    }

    pub fn shell_ciura(&mut self) {
        let mut ciura = vec![1, 4, 10, 23, 57, 132, 301, 701];
        let mut last = 701;
        while last < self.data.len() / 2 {
            last = ((last as f64) * 2.25) as _;
            ciura.push(last);
        }
        for gap in ciura.iter().rev() {
            self.perform_shell(*gap);
        }
    }

    fn perform_shell(&mut self, gap: usize) {
        let mut i = 0;
        let size = self.data.len();
        while i + gap < size {
            let mut j = i + gap;
            let x = self.data[j];
            while j >= gap && self.data[j - gap] > x {
                self.cmp += 1;
                self.asg += 1;
                self.data[j] = self.data[j - gap];
                j -= gap;
            }
            self.asg += 1;
            self.data[j] = x;
            i += 1;
        }
    }

    pub fn sorted_percent(&self) -> usize {
        let (success, last) =
            self.data
                .iter()
                .skip(1)
                .fold((0, &self.data[0]), |(success, prev), current| {
                    (if prev < current { success + 1 } else { success }, current)
                });
        assert_eq!(self.data.last(), Some(last));
        success * 100 / (self.data.len() - 1)
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

    fn insort_left(&mut self, value: i64, a: usize, b: usize) -> usize {
        let mut low = a;
        let mut high = b;
        loop {
            self.cmp += 1;
            if low < high {
                let mid = low + (high - low) / 2;
                if self.data[mid] < value {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            } else {
                break low;
            }
        }
    }

    fn perform_insertion(&mut self, from: usize, to: usize) {
        assert!(from < self.data.len());
        assert!(to <= from);
        let x = self.data[from];
        for i in (to..from).rev() {
            self.asg += 1;
            self.data[i + 1] = self.data[i];
        }
        self.asg += 1;
        self.data[to] = x;
    }
}
