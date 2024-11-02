use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WeeklyIncomeExpenses{
    pub income:Vec<f32>,
    pub expenses:Vec<f32>
}