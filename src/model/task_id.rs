use uuid::Uuid;

pub struct TaskId(Uuid);

impl TaskId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}