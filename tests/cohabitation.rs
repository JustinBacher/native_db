use native_db::{native_db, ToInput};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db(primary_key(custom_id -> u32))]
struct ItemPK {
    id: u32,
}

impl ItemPK {
    pub fn custom_id(&self) -> u32 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db(
    secondary_key(custom_name -> String),
    secondary_key(custom_name_op -> Option<String>, optional))
]
struct ItemSK {
    #[primary_key]
    id: u32,
    name: String,
    #[secondary_key(optional)]
    name_op: Option<String>,
}
// TODO: Test for other type enum tuple etc ...

impl ItemSK {
    pub fn custom_name(&self) -> String {
        self.name.clone()
    }

    pub fn custom_name_op(&self) -> Option<String> {
        self.name_op.clone()
    }
}

#[test]
fn test_insert_my_item_sk() {
    let item = ItemSK {
        id: 1,
        name: "test".to_string(),
        name_op: Some("test".to_string()),
    };

    let key = item.native_db_primary_key();
    assert_eq!(key, native_db::ToKey::to_key(&1_u32));
    // TODO: changer la macro pour pouvoir utiliser native_db_8_1 aussi.
    // assert_eq!(key, native_db_8_1::ToKey::to_key(&1_u32));
}
