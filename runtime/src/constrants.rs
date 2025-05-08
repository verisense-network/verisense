pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// Time.
pub mod time {
    use vrs_primitives::BlockNumber;
    pub const MILLISECS_PER_BLOCK: u64 = 6000;
    pub const SECS_PER_BLOCK: u64 = MILLISECS_PER_BLOCK / 1000;
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
    pub const EPOCH_DURATION_IN_SLOTS: u64 = {
        const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;
        (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
    };
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
    pub const SESSION_IN_BLOCKS: BlockNumber = 4 * HOURS;
    pub const SESSION_PER_ERA: u32 = 6;
}
