use crate::sort_playground::SortPlayground;

pub fn heap_sort(mut heap: SortPlayground) -> SortPlayground {
    let size = heap.data.len();
    for h in (0..(size / 2)).rev() {
        heapify(&mut heap, h, size);
    }
    for j in (1..size).rev() {
        heap.swap(0, j);
        heapify(&mut heap, 0, j);
    }
    heap
}

fn heapify(heap: &mut SortPlayground, root: usize, size: usize) {
    let mut x = root;
    let l = 2 * x + 1;
    let r = l + 1;
    let a = &heap.data;
    if l < size && a[x] < a[l] {
        x = l;
    }
    if r < size && a[x] < a[r] {
        x = r;
    }
    if x != root {
        heap.swap(root, x);
        heapify(heap, x, size);
    }
}
