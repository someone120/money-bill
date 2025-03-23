use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WeeklyIncomeExpenses{
    pub income:Vec<f32>,
    pub expenses:Vec<f32>
}
#[derive(Serialize,Deserialize)]
pub struct AccountIconName {
    pub name:String,
    pub icon:String
}

#[derive(Serialize,Deserialize)]
pub struct AccountAmount {
    pub account:String,
    pub amount:f32
}