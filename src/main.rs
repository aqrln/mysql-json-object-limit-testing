use itertools::Itertools;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    // With 5_000_000 fields (10_000_000 args):
    //  - Connection reset by peer (os error 54) on MySQL 8.0.35
    //  - Broken pipe (os error 32) on MySQL 8.2.0
    //
    // With 3_200_000 fields (6_400_000 args):
    //  - 1153 (08S01): Got a packet bigger than 'max_allowed_packet' bytes
    let json: (serde_json::Value,) = sqlx::query_as(&build_query(3_000_000))
        .fetch_one(&pool)
        .await?;

    dbg!(json.0);

    Ok(())
}

fn build_query(num_fields: u32) -> String {
    let args = (1..=num_fields)
        .map(|i| format!("'{i}'"))
        .flat_map(|x| std::iter::repeat(x).take(2))
        .join(", ");

    let query = format!("SELECT JSON_OBJECT({args})");

    dbg!(query)
}
