use datafusion::error::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {

    let context = SessionContext::new();
    //let test_data = datafusion::test_util::parquet_test_data();
    let test_data = "/mnt/c/Dev/Rust/archive";

    context.register_parquet(
        "patient",
        &format!("{test_data}/Patient.parquet"),
        ParquetReadOptions::default(),
    ).await?;

    let df = context.sql(
        "explain select * from patient"
    ).await?;

    df.show().await?;

    Ok(())
}
