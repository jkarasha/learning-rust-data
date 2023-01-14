use datafusion::error::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let context = SessionContext::new();
    //let test_data = datafusion::test_util::arrow_test_data();
    let test_data = "../testing/data";
    //
    context.register_csv(
        "aggregate_test_1oo",
        &format!("{test_data}/csv/bing_covid-19_data.csv"),
        CsvReadOptions::new(),
    ).await?;

    let df = context.sql(
        "select * from bing_covid-19_data"
    ).await?;

    df.show().await?;

    Ok(())

}