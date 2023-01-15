use datafusion::error::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {

    let context = SessionContext::new();
    let test_data = datafusion::test_util::parquet_test_data();

    context.register_parquet(
        "alltypes_plain",
        &format!("{test_data}/alltypes_plain.parquet"),
        ParquetReadOptions::default(),
    ).await?;

    let df = context.sql(
        "select * from alltypes_plain"
    ).await?;

    df.show().await?;

    Ok(())
}
