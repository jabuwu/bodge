use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone)]
pub struct Label(pub String);

impl Label {
    pub fn find(label_query: &Query<(Entity, &Label)>, str: &str) -> Entity {
        Label::try_find(label_query, str).expect(&format!("failed to find label: {}", str))
    }

    pub fn try_find(label_query: &Query<(Entity, &Label)>, str: &str) -> Option<Entity> {
        for (entity, label) in label_query.iter() {
            if label.0 == str {
                return Some(entity);
            }
        }
        None
    }

    pub fn new(label: &str) -> Label {
        Label(label.to_owned())
    }
}
