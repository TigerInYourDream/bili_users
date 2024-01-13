use sqlx::{QueryBuilder, Sqlite, SqliteConnection};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_log() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[derive(sqlx::FromRow, Debug)]
pub struct BaseCol {
    pub mid: i64,
    pub lable_theme: String,
    pub name: String,
    pub sex: String,
}

pub async fn insert(conn: &mut SqliteConnection, data: &Vec<BaseCol>) -> anyhow::Result<()> {
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO base (mid,lable_theme,name,sex) ");

    query_builder.push_values(data.iter().take(data.len()), |mut b, user| {
        b.push_bind(user.mid)
            .push_bind(user.lable_theme.clone())
            .push_bind(user.name.clone())
            .push_bind(user.sex.clone());
    });

    let query_sql = query_builder.build();
    query_sql.execute(conn).await?;
    Ok(())
}

pub async fn last_mid(conn: &mut SqliteConnection) -> anyhow::Result<i64> {
    let rows: (String,) = sqlx::query_as(
        "SELECT mid
        FROM base
        ORDER BY mid DESC
        LIMIT 1",
    )
    .fetch_one(conn)
    .await?;
    let mid = rows.0.parse::<i64>()?;
    Ok(mid)
}

#[cfg(test)]
mod test {
    use sqlx::{Connection, SqliteConnection};

    #[tokio::test]
    async fn test_last_mid() -> anyhow::Result<()> {
        let mut conn = SqliteConnection::connect("./source/userinfo_db").await?;
        let rows: (String,) = sqlx::query_as(
            "SELECT mid
            FROM base
            ORDER BY mid DESC
            LIMIT 1",
        )
        .fetch_one(&mut conn)
        .await?;
        println!("{rows:?}");
        Ok(())
    }
}
