pub fn clamp_board_idx(val: i32) -> usize {
    val.max(0).min(7) as usize
}
