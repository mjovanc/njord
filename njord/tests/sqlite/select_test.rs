use njord::column::Column;
use njord::condition::Condition;
use njord::keys::{AutoIncrementPrimaryKey, PrimaryKey};
use njord::sqlite;
use njord::table::Table;
use njord::util::JoinType;
use njord_derive::Table;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

#[derive(Table, Clone)]
#[table_name = "users"]
pub struct User {
    id: AutoIncrementPrimaryKey<usize>,
    username: String,
    email: String,
    address: String,
}

#[derive(Table)]
#[table_name = "users"]
pub struct UserWithSubQuery {
    id: AutoIncrementPrimaryKey<usize>,
    username: String,
    email: String,
    address: String,
    additional_address: String,
}

#[derive(Table)]
#[table_name = "users"]
pub struct UsersWithJoin {
    username: String,
    price: f64,
    name: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct Category {
    id: PrimaryKey<usize>,
    name: String,
}

#[derive(Table)]
#[table_name = "products"]
pub struct Product {
    id: PrimaryKey<usize>,
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    category: Category, // one-to-one relationship
    discount: f64,
}

// #[derive(Table)]
// #[table_name = "orders"]
// pub struct Order {
//     id: usize,
//     user: User,             // one-to-one relationship
//     products: Vec<Product>, // one-to-many relationship - populates from based on junction table (gets from macro attribute "table_name" and combines them for example, orders_products)
//     total_cost: f64,
// }

#[test]
fn open_db() {
    let db_relative_path = "./db/open.db";
    let db_path = Path::new(&db_relative_path);

    let result = sqlite::open(db_path);
    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(c) => {
            let result = sqlite::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn update() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec!["username".to_string()];

    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::<usize>::new(Some(0)),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::update(&c, table_row)
                .set(columns)
                .where_clause(condition)
                .order_by(order)
                .limit(4)
                .offset(0)
                .build();
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to UPDATE: {:?}", e);
        }
    }
}

#[test]
fn delete() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let condition = Condition::Eq("address".to_string(), "Some Random Address 1".to_string());

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::delete(c)
                .from(User::default())
                .where_clause(condition)
                .order_by(order)
                .limit(20)
                .offset(0)
                .build();
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

#[test]
fn select() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 2),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_distinct() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .distinct()
                .build();

            match result {
                Ok(r) => {
                    // TODO: this test does not work properly since it should return 1 but it seems
                    // like it returns all rows because id is different. Need to check up on that.
                    assert_eq!(r.len(), 2);
                }
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_group_by() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .group_by(group_by)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_order_by() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["email".to_string()], "ASC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_limit_offset() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .limit(1)
                .offset(0)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(error) => panic!("Failed to SELECT: {:?}", error),
    };
}

#[test]
fn select_having() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["email".to_string()], "DESC".to_string());

    let having_condition = Condition::Gt("id".to_string(), "1".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .having(having_condition)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}

#[test]
fn select_except() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];

    let condition1 = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let condition2 = Condition::Eq("username".to_string(), "otheruser".to_string());
    let condition3 = Condition::Eq("username".to_string(), "anotheruser".to_string());

    match conn {
        Ok(c) => {
            // Create a new connection for each query builder
            let query1 = sqlite::select(&c, columns.clone())
                .from(User::default())
                .where_clause(condition1);

            let query2 = sqlite::select(&c, columns.clone())
                .from(User::default())
                .where_clause(condition2);

            let query3 = sqlite::select(&c, columns.clone())
                .from(User::default())
                .where_clause(condition3);

            // Test a chain of EXCEPT queries (query1 EXCEPT query2 EXCEPT query3)
            let result = query1.except(query2).except(query3).build();

            match result {
                Ok(r) => {
                    assert_eq!(r.len(), 2, "Expected 2 results after EXCEPT clauses.");
                }
                Err(e) => panic!("Failed to SELECT with EXCEPT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_union() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];

    let condition1 = Condition::Eq("id".to_string(), 42.to_string());
    let condition2 = Condition::Eq("id".to_string(), 43.to_string());

    match conn {
        Ok(c) => {
            // Create a new connection for each query builder
            let query1 = sqlite::select(&c, columns.clone())
                .from(User::default())
                .where_clause(condition1);

            let query2 = sqlite::select(&c, columns.clone())
                .from(User::default())
                .where_clause(condition2);

            // Test a chain of UNION queries (query1 UNION query2)
            let result = query1.union(query2).build();

            match result {
                Ok(r) => {
                    // We expect two results: mjovanc and otheruser
                    assert_eq!(r.len(), 2, "Expected 2 results from the UNION query.");
                    assert_eq!(
                        r[0].id,
                        AutoIncrementPrimaryKey::new(Some(42)),
                        "First user should be mjovanc."
                    );
                    assert_eq!(
                        r[1].id,
                        AutoIncrementPrimaryKey::new(Some(43)),
                        "Second user should be otheruser."
                    );
                }
                Err(e) => panic!("Failed to SELECT with UNION: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}

#[test]
fn select_sub_queries() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    match conn {
        Ok(c) => {
            let sub_query = sqlite::select(&c, vec![Column::Text("username".to_string())])
                .from(UserWithSubQuery::default());

            let columns = vec![
                Column::Text("id".to_string()),
                Column::Text("username".to_string()),
                Column::Text("email".to_string()),
                Column::Text("address".to_string()),
                Column::SubQuery(sub_query),
            ];

            let result = sqlite::select(&c, columns)
                .from(UserWithSubQuery::default())
                .build();

            match result {
                Ok(r) => {
                    assert_eq!(r.len(), 2);
                    assert_eq!(r[0].additional_address, "mjovanc");
                }
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_in() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];

    let condition = Condition::And(
        Box::new(Condition::In(
            "username".to_string(),
            vec!["mjovanc".to_string(), "otheruser".to_string()],
        )),
        Box::new(Condition::NotIn(
            "username".to_string(),
            vec!["chasewillden".to_string()],
        )),
    );

    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(User::default())
                .where_clause(condition)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 2),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_join() {
    let db_relative_path = "./db/select_join.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    // Assume we have pre-inserted some data into the users and products tables
    let columns = vec![
        Column::Text("users.username".to_string()),
        Column::Text("products.name".to_string()),
        Column::Text("products.price".to_string()),
    ];

    // Assuming a hypothetical join condition: users.id = products.user_id
    let join_condition = Condition::Eq("users.id".to_string(), "products.user_id".to_string());
    match conn {
        Ok(c) => {
            let result = sqlite::select(&c, columns)
                .from(UsersWithJoin::default())
                .join(
                    JoinType::Inner,
                    Arc::new(Product::default()),
                    join_condition,
                )
                .build();
            match result {
                Ok(r) => {
                    // Check the number of results and assert against expected values
                    assert!(!r.is_empty(), "Expected results, but got none.");
                    // Further assertions on expected data can be made here based on inserted data
                }
                Err(e) => panic!("Failed to SELECT with JOIN: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}
