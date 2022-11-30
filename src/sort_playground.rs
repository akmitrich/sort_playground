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
        let a: Vec<_> = (0..n as i64).rev().collect();
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
            "{} numbers. Sorted {}% with {} comparisons and {} assignments.",
            self.data.len(),
            crate::sorted_percent(self.iter()),
            self.cmp,
            self.asg
        )
    }

    pub(crate) fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.data.len());
        assert!(b < self.data.len());
        self.data.swap(a, b);
        self.asg += 3; // Suppose we make 3 assignments during swap
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
