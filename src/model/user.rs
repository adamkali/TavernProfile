// Tavern Imports
use tavern_common::data_structures::model::Model;

// 3rd party stuff
use uuid::Uuid;

#[derive(Model, Serialize, Clone)]
pub struct UserModel {
    pub user_id: String,
    pub user_bio: String,
    pub username: String,
    // pub user_characters: Vec<Character>
    // pub user_plots: Vec<Plot>
}

impl UserModel {
    pub fn new(id: Option<String>) -> Self {
        match id {
            Some(i) => Self {
                user_id: i,
                username: "Default Username".to_string(),
                user_bio: "Add in a bio to help people get to know you!".to_string(),
            },
            None => Self {
                user_id: Uuid::new_v4().to_string(),
                username: "Default Username".to_string(),
                user_bio: "Add in a bio to help people get to know you!".to_string(),
            }
        }
    }

    pub fn size(&self) -> u64 {
        let mut size: u64 = 0;
        size = (self.user_id.len() + self.user_bio.len() + self.username.len()) as u64;
        return size;
    }

    pub fn create_with_id_username(
        id: String, 
        username: String
    ) -> Self {
        Self {
            user_id: id,
            username,
            user_bio: "Add in a bio to help people get to know you!".to_string(),
        }
    }
}
