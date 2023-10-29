pub fn clamp_board_idx(val: i32) -> usize {
    val.max(0).min(8) as usize
}
