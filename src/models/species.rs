use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::color::{Color, ColorId};
use crate::models::generation::{Generation, GenerationId};
use crate::models::growth_rate::{GrowthRate, GrowthRateId};
use crate::models::habitat::{Habitat, HabitatId};
use crate::models::item::{Item, ItemId};
use crate::models::language::LanguageId;
use crate::models::localized_names::LocalizedStrings;
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::evolution::{Evolution, UnlinkedEvolution};
use crate::models::version::VersionId;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use crate::types::capture_rate::CaptureRate;
use crate::types::gender_rate::GenderRate;
use crate::types::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod evolution;

pub type SpeciesId = u16;

#[derive(Debug)]
pub struct Species {
    pub id: SpeciesId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub flavor_texts: SpeciesFlavorTexts,
    pub form_description: Option<LocalizedStrings>,
    pub generation: Arc<Generation>,
    pub evolves_from_species_id: Option<SpeciesId>,
    pub baby_trigger_item: Option<Arc<Item>>,
    pub evolutions: Vec<Evolution>,
    pub color: Arc<Color>,
    pub shape: Arc<Shape>,
    pub habitat: Option<Arc<Habitat>>,
    pub gender_rate: GenderRate,
    pub capture_rate: CaptureRate,
    pub base_happiness: u8,
    pub is_baby: bool,
    pub hatch_counter: u8,
    pub has_gender_differences: bool,
    pub growth_rate: Arc<GrowthRate>,
    pub forms_switchable: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub order: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedSpecies {
    pub id: SpeciesId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub flavor_texts: SpeciesFlavorTexts,
    pub form_description: Option<LocalizedStrings>,
    pub generation_id: GenerationId,
    pub evolves_from_species_id: Option<SpeciesId>,
    pub baby_trigger_item_id: Option<ItemId>,
    pub evolutions: Vec<UnlinkedEvolution>,
    pub color_id: ColorId,
    pub shape_id: ShapeId,
    pub habitat_id: Option<HabitatId>,
    pub gender_rate: GenderRate,
    pub capture_rate: CaptureRate,
    pub base_happiness: u8,
    pub is_baby: bool,
    pub hatch_counter: u8,
    pub has_gender_differences: bool,
    pub growth_rate_id: GrowthRateId,
    pub forms_switchable: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub order: u16,
}

impl Linkable for UnlinkedSpecies {
    type Linked = Arc<Species>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let generation = context
            .generations
            .get(&self.generation_id)
            .unwrap_or_else(|| {
                panic!(
                    "No generation '{}' found for species '{}'",
                    self.generation_id, self.id
                )
            })
            .clone();

        let baby_trigger_item = self.baby_trigger_item_id.map(|id| {
            context
                .items
                .get(&id)
                .unwrap_or_else(|| {
                    panic!(
                        "No baby trigger item '{}' found for species '{}'",
                        id, self.id
                    )
                })
                .clone()
        });

        let evolutions = self
            .evolutions
            .iter()
            .map(|evolution| evolution.link(context))
            .collect();

        let color = context
            .colors
            .get(&self.color_id)
            .unwrap_or_else(|| {
                panic!(
                    "No color '{}' found for species '{}'",
                    self.color_id, self.id
                )
            })
            .clone();

        let shape = context
            .shapes
            .get(&self.shape_id)
            .unwrap_or_else(|| {
                panic!(
                    "No shape '{}' found for species '{}'",
                    self.shape_id, self.id
                )
            })
            .clone();

        let habitat = self.habitat_id.map(|id| {
            context
                .habitats
                .get(&id)
                .unwrap_or_else(|| panic!("No habitat '{}' found for species '{}'", id, self.id))
                .clone()
        });

        let growth_rate = context
            .growth_rates
            .get(&self.growth_rate_id)
            .unwrap_or_else(|| {
                panic!(
                    "No growth rate '{}' found for species '{}'",
                    self.growth_rate_id, self.id
                )
            })
            .clone();

        let species = Species {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            flavor_texts: self.flavor_texts.clone(),
            form_description: self.form_description.clone(),
            generation,
            evolves_from_species_id: self.evolves_from_species_id,
            baby_trigger_item,
            evolutions,
            color,
            shape,
            habitat,
            gender_rate: self.gender_rate,
            capture_rate: self.capture_rate,
            base_happiness: self.base_happiness,
            is_baby: self.is_baby,
            hatch_counter: self.hatch_counter,
            has_gender_differences: self.has_gender_differences,
            growth_rate,
            forms_switchable: self.forms_switchable,
            is_legendary: self.is_legendary,
            is_mythical: self.is_mythical,
            order: self.order,
        };

        Arc::new(species)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesFlavorTexts(HashMap<LanguageId, Vec<SpeciesFlavorText>>);

impl SpeciesFlavorTexts {
    pub fn new(texts: HashMap<LanguageId, Vec<SpeciesFlavorText>>) -> Self {
        Self(texts)
    }

    pub fn get_by_language(&self, language: Language) -> Option<&Vec<SpeciesFlavorText>> {
        let language_id = language as LanguageId;
        if let Some(target) = self.0.get(&language_id) {
            return Some(target);
        }

        let default_language_id = Language::default() as LanguageId;
        if let Some(default) = self.0.get(&default_language_id) {
            return Some(default);
        }

        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesFlavorText {
    pub version_id: VersionId,
    pub flavor_text: String,
}

impl HasIdentifier for Species {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl HasLocalizedNames for Species {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}
