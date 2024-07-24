use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Actor<'a> {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "StageName")]
    stage_name: &'a str,
    #[serde(rename = "FullName")]
    pub full_name: &'a str,
    #[serde(rename = "KoreanName")]
    korean_name: &'a str,
    #[serde(rename = "KoreanStageName")]
    korean_stage_name: &'a str,
    #[serde(rename = "DateOfBirth")]
    date_of_birth: &'a str,
    #[serde(rename = "Country")]
    country: &'a str,
    #[serde(rename = "Birthplace")]
    birthplace: &'a str,
    #[serde(rename = "Gender")]
    gender: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Artist<'a> {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "StageName")]
    stage_name: &'a str,
    #[serde(rename = "FullName")]
    pub full_name: &'a str,
    #[serde(rename = "KoreanName")]
    korean_name: &'a str,
    #[serde(rename = "KoreanStageName")]
    korean_stage_name: &'a str,
    #[serde(rename = "DateOfBirth")]
    date_of_birth: &'a str,
    #[serde(rename = "Group")]
    group: &'a str,
    #[serde(rename = "Country")]
    country: &'a str,
    #[serde(rename = "Birthplace")]
    birthplace: &'a str,
    #[serde(rename = "2ndGroup")]
    second_group: &'a str,
    #[serde(rename = "Gender")]
    gender: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Group<'a> {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: &'a str,
    #[serde(rename = "ShortName")]
    short_name: &'a str,
    #[serde(rename = "KoreanName")]
    korean_name: &'a str,
    #[serde(rename = "Debut")]
    debut: &'a str,
    #[serde(rename = "Company")]
    company: &'a str,
    #[serde(rename = "CurrentMemberCount")]
    current_member_count: u32,
    #[serde(rename = "OriginalMemberCount")]
    original_member_count: u32,
    #[serde(rename = "FanbaseName")]
    fanbase_name: &'a str,
    #[serde(rename = "Active")]
    active: &'a str,
}
