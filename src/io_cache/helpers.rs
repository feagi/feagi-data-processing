// TODO have these structures better implement comparison logic without having to use index()

/// The 
#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct GroupingIndex {
    index: usize,
}
impl GroupingIndex {
    pub fn new(index: usize) -> Self {
        GroupingIndex { index }
    }
    
    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct ChannelIndex {
    index: usize,
}
impl ChannelIndex {
    pub fn new(index: usize) -> Self {
        ChannelIndex { index }
    }
    
    pub fn index(&self) -> usize {
        self.index
    }
}

impl std::fmt::Display for ChannelIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Channel: {}", self.index)
    }
}

pub enum ChannelStatus {
    Enabled,
    Unused,
    Disabled,
}