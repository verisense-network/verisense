use vrs_core_sdk::{get, init, post, set_timer, storage, timer};
#[init]
pub fn timer_init() {
    test_set_perfect_tree_mod_timer(1, 0);
}
#[timer]
pub fn test_delay(a: String, b: i32) {
    storage::put(b"delay", format!("delay_complete {} {}", a, b).as_bytes()).unwrap();
}

#[post]
pub fn test_set_timer() {
    storage::put(b"delay", format!("init").as_bytes());

    let a = "abc".to_string();
    let b = 123;
    set_timer!(std::time::Duration::from_secs(4), test_delay(a, b));
}
#[get]
pub fn test_get_timer() -> Result<String, String> {
    let res = storage::get(b"delay").map_err(|e| e.to_string())?.unwrap();
    Ok(String::from_utf8(res).unwrap())
}
#[post]
pub fn test_set_tree_mod_timer() {
    test_delay("init".to_string(), 0);
    for i in (1..=10).rev() {
        set_timer!(
            std::time::Duration::from_secs(i),
            test_delay("abc".to_string(), i as i32)
        )
        .unwrap();
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
        storage::put(b"tree", format!("node {} using time {}", 1, 0).as_bytes())
            .map_err(|e| e.to_string())?;
    } else {
        storage::put(
            b"tree",
            format!("node {} using time {}", i, using_time).as_bytes(),
        )
        .map_err(|e| e.to_string())?;
    }
    if i >= 8 {
        return Ok(i);
    }
    set_timer!(
        std::time::Duration::from_secs(1),
        test_set_perfect_tree_mod_timer(i * 2, using_time + 1)
    );
    set_timer!(
        std::time::Duration::from_secs(2),
        test_set_perfect_tree_mod_timer(i * 2 + 1, using_time + 2)
    );

    Ok(i)
}
#[timer]
pub fn test_empty_timer() -> Result<i32, String> {
    Ok(0)
}
#[post]
pub fn test_set_empty_timer() {
    set_timer!(std::time::Duration::from_secs(1), test_empty_timer()).unwrap();
}
