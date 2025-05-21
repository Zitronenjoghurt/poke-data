use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::contest_type::ContestTypeId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestTypeNameData {
    contest_type_id: ContestTypeId,
    pub local_language_id: LanguageId,
    name: String,
    pub flavor: String,
    color: Option<String>,
}

impl PokeApiModel for ContestTypeNameData {
    fn file_name() -> &'static str {
        "contest_type_names"
    }
}

impl HasId for ContestTypeNameData {
    type Id = ContestTypeId;

    fn id(&self) -> Self::Id {
        self.contest_type_id
    }
}

impl HasLocalizedString for ContestTypeNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
