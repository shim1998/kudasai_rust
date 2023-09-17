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

pub fn modify_user<'a>(action:&'a str,db_name:&'a str,user_name:&'a str,user_id:&'a str) -> Result<(),&'a str>{
    let connection = sqlite::open(db_name).unwrap();
    let query = format!("SELECT COUNT(*) FROM users WHERE client_id={};",user_id);
    let mut flag=1;
    connection
    .iterate(query, |pairs| {
        for &(_,value) in pairs.iter() {
            if value.unwrap() != "0"{
                flag=0;
            }
        }
        true
    }).unwrap();
    if action == "add" {
        if flag == 0 {
            return Err("User already exists")
        }
        let add_statement = format!("INSERT INTO users(name,client_id,score_matrix,points) VALUES('{}','{}','{{}}',0)",user_name,user_id);
        match connection.execute(add_statement){
        Err(e) => error!("Error executing statement: {}",e),
        _ => info!("Executed successfully")
        };
        Ok(())
    } else if action == "delete" {
        let delete_statement = format!("DELETE FROM users WHERE client_id={}",user_id);
        match connection.execute(delete_statement){
        Err(e) => error!("Error executing statement: {}",e),
        _ => info!("Executed successfully")
        };
        Ok(())
    }
    else {
        return Err("Wrong action")
    }
}
