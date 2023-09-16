use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarksApiResponse {
    subjects: Vec<Subject>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    #[serde(alias = "Marks")]
    marks: Vec<Mark>,
    #[serde(alias = "Subject")]
    subject_info: SubjectInfo,
    #[serde(alias = "AverageText")]
    average_text: String

}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectInfo {
    #[serde(alias = "Id")]
    id: u32,
    #[serde(alias = "Abbrev")]
    abbreviation: String,
    #[serde(alias = "Name")]
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mark {

}
