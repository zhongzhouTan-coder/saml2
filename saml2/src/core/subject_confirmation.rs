use super::{
    base_id::BaseID, encrypted_id::EncryptedID, name_id::NameID,
    subject_confirmation_data::SubjectConfirmationData,
};

#[derive(Clone, Debug)]
pub struct SubjectConfirmation {
    base_id: Option<BaseID>,
    name_id: Option<NameID>,
    encrypted_id: Option<EncryptedID>,
    subject_confirmation_data: Option<SubjectConfirmationData>,
    method: String,
}
