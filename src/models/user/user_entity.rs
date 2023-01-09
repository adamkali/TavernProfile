// Tavern Imports
use arctic_fox::arctic_fox_data_structures::cub::Cub;

// 3rd party stuff
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct UserModel {
    pub user_id: String,
    pub user_bio: String,
    pub username: String,
    // pub user_characters: Vec<Character>
    // pub user_plots: Vec<Plot>
}

impl Default for UserModel {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4().to_string(),
            username: "Defaulto".to_string(),
            user_bio: "Add in a bio to help people get to know you!".to_string(),
        }
    }
}

impl Model for UserModel {
    fn new(id: Option<String>) -> Self {
        match id {
            Some(i) => Self {
                user_id: i,
                ..Self::default()
            },
            None => Self {
                user_id: Uuid::new_v4().to_string(),
                username: "Default Username".to_string(),
                user_bio: "Add in a bio to help people get to know you!".to_string(),
            }
        }
    }

    fn size(&self) -> u64 {
        let mut size: u64 = 0;
        size = (self.user_id.len() + self.user_bio.len() + self.username.len()) as u64;
        return size;
    }

}

impl UserModel {
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

