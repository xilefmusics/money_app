use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GoalData {
    #[serde(rename = "realWealth")]
    RealWealth(u32),
}

impl Default for GoalData {
    fn default() -> Self {
        GoalData::RealWealth(0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Goal {
    pub id: Option<String>,
    pub due: DateTime<Local>,
    pub data: GoalData,
}

#[cfg(feature = "backend")]
impl fancy_surreal::Databasable for Goal {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}
