use openssl::ssl::{SslConnector, SslMethod};
use postgres::types::Type;
use postgres_openssl::MakeTlsConnector;
use rand::Rng;

fn main() {
    let pg_host =
        std::env::var("POSTGRES_HOST").expect("missing environment variable POSTGRES_HOST");
    let pg_user =
        std::env::var("POSTGRES_USER").expect("missing environment variable POSTGRES_USER");
    let pg_password =
        std::env::var("POSTGRES_PASSWORD").expect("missing environment variable POSTGRES_PASSWORD");
    let pg_dbname = std::env::var("POSTGRES_DBNAME").unwrap_or("postgres".to_string());

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let tls_connector = MakeTlsConnector::new(builder.build());

    let url = format!(
        "host={} port=5432 user={} password={} dbname={} sslmode=require",
        pg_host, pg_user, pg_password, pg_dbname
    );
    let mut pg_client =
        postgres::Client::connect(&url, tls_connector).expect("failed to connect to postgres");

    drop_create_table(&mut pg_client);
    insert_data(&mut pg_client);
    query_data(&mut pg_client);
    update_data(&mut pg_client);
    delete(&mut pg_client);

    pg_client.close().expect("failed to close client");
}

const CREATE_QUERY: &str =
    "CREATE TABLE inventory (id serial PRIMARY KEY, name VARCHAR(50), quantity INTEGER);";

const DROP_TABLE: &str = "DROP TABLE inventory";

fn drop_create_table(pg_client: &mut postgres::Client) {
    let res = pg_client.execute(DROP_TABLE, &[]);
    match res {
        Ok(_) => println!("dropped 'inventory' table"),
        Err(e) => println!("failed to drop 'inventory' table {}", e),
    }
    pg_client
        .execute(CREATE_QUERY, &[])
        .expect("failed to create 'inventory' table");
}

const INSERT_QUERY: &str = "INSERT INTO inventory (name, quantity) VALUES ($1, $2) RETURNING id;";

fn insert_data(pg_client: &mut postgres::Client) {
    //prepared statement
    let prep_stmt = pg_client
        .prepare(&INSERT_QUERY)
        .expect("failed to create prepared statement");

    let row = pg_client
        .query_one(&prep_stmt, &[&"item-1", &42])
        .expect("insert failed");

    let id: i32 = row.get(0);
    println!("inserted item with id {}", id);

    //typed prepared statement
    let typed_prep_stmt = pg_client
        .prepare_typed(&INSERT_QUERY, &[Type::VARCHAR, Type::INT4])
        .expect("failed to create prepared statement");

    let row = pg_client
        .query_one(&typed_prep_stmt, &[&"item-2", &43])
        .expect("insert failed");

    let id: i32 = row.get(0);
    println!("inserted item with id {}", id);

    for n in 3..=5 {
        let row = pg_client
            .query_one(
                &typed_prep_stmt,
                &[
                    &("item-".to_owned() + &n.to_string()),
                    &rand::thread_rng().gen_range(10..=50),
                ],
            )
            .expect("insert failed");

        let id: i32 = row.get(0);
        println!("inserted item with id {} ", id);
    }
}

const SELECT_ALL_QUERY: &str = "SELECT * FROM inventory;";
const SELECT_BY_ID: &str = "SELECT name, quantity FROM inventory where id=$1;";

fn query_data(pg_client: &mut postgres::Client) {
    let prep_stmt = pg_client
        .prepare_typed(&SELECT_BY_ID, &[Type::INT4])
        .expect("failed to create prepared statement");

    let item_id = 1;
    let c = pg_client
        .query_one(&prep_stmt, &[&item_id])
        .expect("failed to query item");

    let name: String = c.get(0);
    let quantity: i32 = c.get(1);
    println!("quantity for item {} = {}", name, quantity);

    let items = pg_client
        .query(SELECT_ALL_QUERY, &[])
        .expect("select all failed");

    println!("listing items...");
    for item in items {
        let id: i32 = item.get("id");
        let name: String = item.get("name");
        let quantity: i32 = item.get("quantity");
        println!(
            "item info: id = {}, name = {}, quantity = {} ",
            id, name, quantity
        );
    }
}

const UPDATE_QUERY: &str = "UPDATE inventory SET quantity = $1 WHERE name = $2 RETURNING quantity;";

fn update_data(pg_client: &mut postgres::Client) {
    let stmt = pg_client
        .prepare_typed(&UPDATE_QUERY, &[Type::INT4, Type::VARCHAR])
        .expect("failed to create prepared statement");

    for id in 1..=5 {
        let row = pg_client
            .query_one(
                &stmt,
                &[
                    &rand::thread_rng().gen_range(10..=50),
                    &("item-".to_owned() + &id.to_string()),
                ],
            )
            .expect("update failed");

        let quantity: i32 = row.get("quantity");
        println!("updated item id {} to quantity = {}", id, quantity);
    }
}

const DELETE_QUERY: &str = "DELETE FROM inventory WHERE id = $1 RETURNING id,name,quantity;";

fn delete(pg_client: &mut postgres::Client) {
    let stmt = pg_client
        .prepare_typed(&DELETE_QUERY, &[Type::INT4])
        .expect("failed to create prepared statement");

    let item = pg_client
        .query_one(&stmt, &[&rand::thread_rng().gen_range(1..=5)])
        .expect("delete failed");

    let id: i32 = item.get(0);
    let name: String = item.get(1);
    let quantity: i32 = item.get(2);
    println!(
        "deleted item info: id = {}, name = {}, quantity = {} ",
        id, name, quantity
    );
}
