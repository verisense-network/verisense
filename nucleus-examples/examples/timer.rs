use vrs_core_sdk::{get, init, post, set_timer, storage, timer, timer_init};
#[timer_init]
pub fn timer_init() {
    test_set_perfect_tree_mod_timer(1, 0);
}
#[timer]
pub fn test_delay(a: String, b: i32) {
    storage::put(b"delay", format!("delay_complete {} {}", a, b).as_bytes()).unwrap();
}

#[timer]
pub fn test_set_timer() {
    storage::put(b"delay", format!("init").as_bytes());

    let a = "abc".to_string();
    let b = 123;
    set_timer!(4, test_delay, a, b);
}
#[timer]
pub fn test_stream(i: i32, j: i32) -> Result<i32, String> {
    if i == 1 {
        storage::put(b"delay", format!("node {} using time {}", 1, 0).as_bytes())
            .map_err(|e| e.to_string())?;
    } else {
        storage::put(b"delay", format!("node {} using time {}", i, j).as_bytes())
            .map_err(|e| e.to_string())?;
    }
    set_timer!(1, test_stream, 1, 2);
    Ok(555)
}
#[timer]
pub fn test_set_tree_mod_timer() {
    test_delay("init".to_string(), 0);
    for i in (1..=10).rev() {
        set_timer!(i, test_delay, "abc".to_string(), i as i32).unwrap();
    }
}

//          0
//        /   \
//       1     2
//    /   \    /  \
//   1     2   1   2
//  / \   / \ / \ / \
// 1   2 1  2 1 2 1 2

//            1
//        /        \
//       2          3
//    /   \       /    \
//   4     5      6     7
//  / \   / \    / \   / \
// 8   9 10  11 12 13 14 15
#[timer]
pub fn test_set_perfect_tree_mod_timer(i: i32, using_time: i32) -> Result<i32, String> {
    if i == 1 {
        storage::put(b"delay", format!("node {} using time {}", 1, 0).as_bytes())
            .map_err(|e| e.to_string())?;
    } else {
        storage::put(
            b"delay",
            format!("node {} using time {}", i, using_time).as_bytes(),
        )
        .map_err(|e| e.to_string())?;
    }
    if i >= 8 {
        return Ok(i);
    }
    set_timer!(1, test_set_perfect_tree_mod_timer, i * 2, using_time + 1);
    set_timer!(
        2,
        test_set_perfect_tree_mod_timer,
        i * 2 + 1,
        using_time + 2
    );

    Ok(i)
}
