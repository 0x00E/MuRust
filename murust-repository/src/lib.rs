#[cfg(test)]
extern crate tempdir;

#[macro_use]
extern crate diesel;
extern crate boolinator;

pub use self::context::DataContext;
pub use self::repository::*;

mod context;
pub mod object;
mod repository;
mod schema;
mod types;
mod util;

#[cfg(test)]
mod tests {
  use super::*;
  use tempdir::TempDir;
  use types::Id;

  // TODO: Share this between crates somehow?
  fn setup_test_db() -> (TempDir, DataContext) {
    let tmp = TempDir::new("murust-repository").expect("creating tempdir");
    let path_buf = tmp.path().join("database.sqlite");
    let path = path_buf.to_str().expect("converting temp DB path");

    let database = DataContext::new(path).expect("creating DB");
    database
      .initialize_schema()
      .expect("creating default schema");
    database.initialize_data().expect("creating test data");

    (tmp, database)
  }

  #[test]
  fn find_account_by_username_and_id() {
    let (_temp, db) = setup_test_db();
    let accounts = AccountRepository::new(&db);

    assert!(accounts.find_by_username("foobar").unwrap().is_some());
    assert!(accounts.find_by_id(&1).unwrap().is_some());
  }

  #[test]
  fn add_and_then_remove_account() {
    let (_temp, db) = setup_test_db();
    let accounts = AccountRepository::new(&db);

    let account = accounts
      .create(
        "fajbar",
        "$2y$07$zFM0q8YmKjaYW4Hig6AFz.wroa/eG5DSK4ST9Y0KS4hDw5Jepw31a",
        123456,
        "fajbar@mail.com",
      )
      .unwrap();
    assert!(accounts.delete(&account.id).is_ok());
  }

  #[test]
  fn find_character_by_name() {
    let (_temp, db) = setup_test_db();
    let repository = CharacterRepository::new(&db);
    assert!(repository.find_by_name("deadbeef").unwrap().is_some());
  }

  #[test]
  fn find_characters_from_account() {
    let (_temp, db) = setup_test_db();
    let repository = CharacterRepository::new(&db);

    let characters = repository.find_by_account_id(1).unwrap();
    assert_eq!(characters.len(), 1);
    assert_eq!(characters[0].name, "deadbeef");
  }

  #[test]
  fn find_item_by_id_and_update() {
    let (_temp, db) = setup_test_db();
    let repository = ItemRepository::new(&db);

    let id = Id::from_hex("6606af63a93c11e4979700505690798f");
    let mut item = repository.find_by_id(&id).unwrap().unwrap();

    assert_eq!(item.level, 2);
    assert_eq!(item.durability, 20);

    item.level = 3;
    repository.save(&item).unwrap();

    let item = repository.find_by_id(&id).unwrap().unwrap();
    assert_eq!(item.level, 3);
  }

  #[test]
  fn find_inventory_by_id() {
    let (_temp, db) = setup_test_db();
    let repository = InventoryRepository::new(&db);

    let inventory = repository.find_by_id(1).unwrap().unwrap();
    assert_eq!(inventory.width, 8);
    assert_eq!(inventory.height, 8);
    assert_eq!(inventory.money, 1337);
  }

  #[test]
  fn find_inventory_items_from_inventory() {
    let (_temp, db) = setup_test_db();
    let repository = InventoryItemRepository::new(&db);

    let items = repository.find_by_inventory_id(1).unwrap();
    assert_eq!(items.len(), 1);

    let id = Id::from_hex("6606af63a93c11e4979700505690798f");
    assert_eq!(items[0].item_id, id);
  }

  #[test]
  fn find_equipment_items_from_character() {
    let (_temp, db) = setup_test_db();
    let repository = EquipmentItemRepository::new(&db);

    let items = repository.find_by_character_id(1).unwrap();
    assert_eq!(items.len(), 1);

    let id = Id::from_hex("3f06af63a93c11e4979700505690773f");
    assert_eq!(items[0].item_id, id);
  }

  #[test]
  fn find_item_eligible_classes_from_item_definition() {
    let (_temp, db) = setup_test_db();
    let repository = ItemEligibleClassRepository::new(&db);

    let classes = repository.find_by_item_definition_code(2).unwrap();
    assert_eq!(classes.len(), 4);
  }
}
