use serde::Deserialize;
 
#[derive(Deserialize, Clone)]
pub struct NewUserData{
    pub name: String
}
 
#[derive(Deserialize, Clone)]
pub struct NewGroupData{
    pub name: String,
    pub creator_id: i32
}
 
#[derive(Deserialize, Clone)]
pub struct AdminOperationData{
    pub initiator_id: i32,
    pub candidate_id: i32
}
 
#[derive(Deserialize, Clone)]
pub struct UserData{
    pub id: i32
}
 
#[derive(Deserialize, Clone)]
pub struct UpdateData{
    pub title: String
}
