use redis::Commands;
use redis_derive::{FromRedisValue, ToRedisArgs};

#[derive(FromRedisValue, ToRedisArgs, Debug)]
struct MySuperCoolStruct {
    first_field : String,
    second_field : Option<i64>,
    third_field : Vec<String>
}

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let test1 = MySuperCoolStruct{
        first_field : "Hello World".to_owned(),
        second_field : Some(42),
        third_field : vec!["abc".to_owned(), "cba".to_owned()]
    };

    let _ = redis::cmd("HMSET")
        .arg("test1")
        .arg(&test1)
        .query(&mut con)?;

    let db_test1 : MySuperCoolStruct = con.hgetall("test1")?;

    println!("send : {:#?}, got : {:#?}", test1, db_test1);
    Ok(())
}