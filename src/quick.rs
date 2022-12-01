use crate::sort_playground::SortPlayground;
use rand::Rng;

pub fn quick_sort(mut quick: SortPlayground) -> SortPlayground {
    let size = quick.data.len() as isize;
    rec_sort(&mut quick, 0,  size - 1);
    quick
}

fn rec_sort(playground: &mut SortPlayground, low: isize, high: isize) {
    if low < high {
        let pivot = partition_rand(playground, low, high);
        rec_sort(playground, low, pivot - 1);
        rec_sort(playground, pivot + 1, high);
    }
}

#[allow(unused)]
fn partition_high(playground: &mut SortPlayground, low: isize, high: isize) -> isize {
    let mut pivot_index = low - 1;
    let pivot_element = playground.data[high as usize];
    for j in low..high {
        playground.cmp += 1;
        if playground.data[j as usize] < pivot_element {
            pivot_index += 1;
            playground.swap(pivot_index as usize, j as usize);
        }
    }
    pivot_index += 1;
    playground.swap(pivot_index as usize, high as usize);
    pivot_index
}

#[allow(unused)]
fn partition_low(playground: &mut SortPlayground, low: isize, high: isize) -> isize {
    playground.swap(low as _, high as _);
    let mut pivot_index = low - 1;
    let pivot_element = playground.data[high as usize];
    for j in low..high {
        playground.cmp += 1;
        if playground.data[j as usize] < pivot_element {
            pivot_index += 1;
            playground.swap(pivot_index as usize, j as usize);
        }
    }
    pivot_index += 1;
    playground.swap(pivot_index as usize, high as usize);
    pivot_index
}

fn partition_rand(playground: &mut SortPlayground, low: isize, high: isize) -> isize {
    let mut rng = rand::thread_rng();
    let p = rng.gen_range(low..high);
    playground.swap(p as _, high as _);
    let mut pivot_index = low - 1;
    let pivot_element = playground.data[high as usize];
    for j in low..high {
        playground.cmp += 1;
        if playground.data[j as usize] < pivot_element {
            pivot_index += 1;
            playground.swap(pivot_index as usize, j as usize);
        }
    }
    pivot_index += 1;
    playground.swap(pivot_index as usize, high as usize);
    pivot_index
}
