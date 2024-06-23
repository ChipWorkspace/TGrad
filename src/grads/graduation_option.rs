use crate::grads::modality::Modality;
use crate::grads::status::GraduationOptionStatus;

use chrono::NaiveDate;

pub struct GraduationOption {
    pub id: Option<String>,
    pub title: String,
    pub modality: Modality,
    pub status: GraduationOptionStatus,
    pub students: Vec<String>,
    pub directors: Vec<String>,
    pub evaluators: Vec<String>,
    pub date: NaiveDate,
}
