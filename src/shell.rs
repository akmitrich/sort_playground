use crate::sort_playground::SortPlayground;

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
