use rand::Rng;

#[derive(Debug, Default)]
pub struct SortPlayground {
    pub data: Vec<i64>,
    pub cmp: usize,
    pub asg: usize,
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

    pub fn iter(&self) -> impl Iterator<Item = &i64> {
        self.data.iter()
    }

    pub fn get_report(&self) -> String {
        format!(
            "Sorted {}% with {} comparisons and {} assignments.",
            sorted_percent(self.iter()),
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

pub fn sorted_percent<'a>(mut data: impl Iterator<Item = &'a i64>) -> usize {
    let start = data.next().unwrap();
    let (sum, success, _) = data.fold((0, 0, start), |(sum, success, prev), current| {
        (
            sum + 1,
            if prev < current { success + 1 } else { success },
            current,
        )
    });
    success * 100 / sum
}

pub fn bubble_sort(mut bubble: SortPlayground) -> SortPlayground {
    for j in (0..bubble.data.len()).rev() {
        for i in 0..j {
            bubble.cmp += 1;
            if bubble.data[i] > bubble.data[i + 1] {
                bubble.swap(i, i + 1);
            }
        }
    }
    bubble
}

pub fn bubble_opt(mut bubble: SortPlayground) -> SortPlayground {
    for j in (0..bubble.data.len()).rev() {
        let mut may_stop = true;
        for i in 0..j {
            bubble.cmp += 1;
            if bubble.data[i] > bubble.data[i + 1] {
                may_stop = false;
                bubble.swap(i, i + 1);
            }
        }
        if may_stop {
            break;
        }
    }
    bubble
}

pub fn insertion(mut ins: SortPlayground) -> SortPlayground {
    for j in 1..ins.data.len() {
        for i in (0..j).rev() {
            ins.cmp += 1;
            if ins.data[i] <= ins.data[i + 1] {
                break;
            }
            ins.swap(i, i + 1);
        }
    }
    ins
}

pub fn insertion_shift(mut ins: SortPlayground) -> SortPlayground {
    for j in 1..ins.data.len() {
        let item = ins.data[j];
        let mut found_index = 0;
        for i in (0..j).rev() {
            ins.cmp += 1;
            if ins.data[i] > item {
                ins.asg += 1;
                ins.data[i + 1] = ins.data[i];
            } else {
                found_index = i + 1;
                break;
            }
        }
        ins.asg += 1;
        ins.data[found_index] = item;
    }
    ins
}

pub fn insertion_binary(mut ins: SortPlayground) -> SortPlayground {
    for j in 1..ins.data.len() {
        let found_index = ins.insort_left(ins.data[j], 0, j);
        ins.perform_insertion(j, found_index);
    }
    ins
}

pub fn shell_naive(mut shell: SortPlayground) -> SortPlayground {
    let mut gap = shell.data.len() / 2;
    while gap > 0 {
        perform_shell(&mut shell, gap);
        gap /= 2;
    }
    shell
}

pub fn shell1(mut shell: SortPlayground) -> SortPlayground {
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
        perform_shell(&mut shell, *gap);
    }
    shell
}

pub fn shell_ciura(mut shell: SortPlayground) -> SortPlayground {
    let mut ciura = vec![1, 4, 10, 23, 57, 132, 301, 701];
    let mut last = 701;
    while last < shell.data.len() / 2 {
        last = ((last as f64) * 2.25) as _;
        ciura.push(last);
    }
    for gap in ciura.iter().rev() {
        perform_shell(&mut shell, *gap);
    }
    shell
}

fn perform_shell(shell: &mut SortPlayground, gap: usize) {
    let mut i = 0;
    let size = shell.data.len();
    while i + gap < size {
        let mut j = i + gap;
        let x = shell.data[j];
        while j >= gap && shell.data[j - gap] > x {
            shell.cmp += 1;
            shell.asg += 1;
            shell.data[j] = shell.data[j - gap];
            j -= gap;
        }
        shell.asg += 1;
        shell.data[j] = x;
        i += 1;
    }
}

pub fn sort<SortMethod>(a: impl Iterator<Item = i64>, method: SortMethod) -> SortPlayground
where
    SortMethod: Fn(SortPlayground) -> SortPlayground,
{
    method(SortPlayground {
        data: a.collect(),
        ..Default::default()
    })
}

impl From<Vec<i64>> for SortPlayground {
    fn from(a: Vec<i64>) -> Self {
        Self {
            data: a,
            ..Default::default()
        }
    }
}

impl From<SortPlayground> for Vec<i64> {
    fn from(val: SortPlayground) -> Self {
        val.data
    }
}

impl IntoIterator for SortPlayground {
    type Item = i64;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
