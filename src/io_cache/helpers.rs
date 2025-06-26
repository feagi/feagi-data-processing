pub struct GroupingIndex {
    index: usize,
}
impl GroupingIndex {
    pub fn new(index: usize) -> Self {
        GroupingIndex { index }
    }
}

pub struct ChannelIndex {
    index: usize,
}
impl ChannelIndex {
    pub fn new(index: usize) -> Self {
        ChannelIndex { index }
    }
}

pub enum ChannelStatus {
    Enabled,
    Unused,
    Disabled,
}