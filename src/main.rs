use tokio_postgres::{NoTls, Error};
use std::time::{Duration, Instant};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=test user=postgres password=example port=54320", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    let n1 = 1000;
    let n2 = 1000;

    for i in 0..n1 {
        let start = Instant::now();
        for j in 0..n2 {
            let name = ((i+1)*(j+1)).to_string();
            let _ = client.execute("insert into test1 (field1) values($1);", &[&name],).await?;
        }

        let e = start.elapsed();
        let n = n2;
        let s_per_req = e/n;
        let req_per_s = (n as f32)/e.as_secs_f32();
        eprintln!("elapsed: {} in {:?}, {:?} seconds/req, {:?} req/sec", n, e, s_per_req, req_per_s);
    }



    Ok(())
}
