use crate::core::Day;
use crate::backend::TimeTableBackend;

#[derive(Clone, Copy, Debug)]
pub struct TimeTableProvider {
    backend: TimeTableBackend,
}

impl TimeTableProvider {
    pub fn new() -> Self {
        Self {
            backend: TimeTableBackend{},
        }
    }

    pub async fn get_formatted(&self, day: Day) -> Result<String, Box<dyn std::error::Error>> {
        let tt = self.backend.get_timetable(day).await?;
        Ok(tt.to_string())
    }
}
