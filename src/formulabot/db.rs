use tracing::{info,error};

pub fn fetch_data(db_name:&str,table_name:&str) {
	let connection = sqlite::open(db_name).unwrap();
	let query = format!("SELECT * FROM {};",table_name);
	connection
    .iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    })
    .unwrap();
}

pub fn add_user(db_name:&str,user_name:&str,user_id:&str){
    let connection = sqlite::open(db_name).unwrap();
    let statement = format!("INSERT INTO users(name,client_id,score_matrix,points) VALUES('{}','{}','{{}}',0)",user_name,user_id);
    match connection.execute(statement){
        Err(e) => error!("ERROR EXECUTING STATEMENT: {}",e),
        _ => info!("User added successfully")
    };
}
