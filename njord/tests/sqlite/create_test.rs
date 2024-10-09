use super::User;
use njord::sqlite;
use std::path::Path;

#[test]
fn create_table() {
    let db_relative_path = "./db/create.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    match conn {
        Ok(c) => {
            let result = sqlite::create::create_table(c, User::default()).build();
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
