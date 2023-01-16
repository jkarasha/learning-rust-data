use datafusion::error::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let context = SessionContext::new();
    //let test_data = datafusion::test_util::arrow_test_data();
    let test_data = "/mnt/c/Dev/Rust/archive";
    //
    context.register_csv(
        "oj_data",
        &format!("{test_data}/oj.csv"),
        CsvReadOptions::new(),
    ).await?;

    let df = context.sql(
        "select * from oj_data"
    ).await?;

    df.show().await?;

    Ok(())

}