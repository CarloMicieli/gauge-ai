use tokio::sync::mpsc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiTask {
    Normalize(String),
    Merge(String),
    Query(String),
}

pub type AiTaskSender = mpsc::Sender<AiTask>;
pub type AiTaskReceiver = mpsc::Receiver<AiTask>;

/// Create a bounded AI task queue to serialize expensive model operations.
pub fn channel(capacity: usize) -> (AiTaskSender, AiTaskReceiver) {
    mpsc::channel(capacity)
}
