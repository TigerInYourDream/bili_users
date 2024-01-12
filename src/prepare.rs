use futures::TryStreamExt;
use sqlx::{QueryBuilder, Sqlite, SqliteConnection};

pub fn init_log() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
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
    let mut rows = sqlx::query_as::<_, BaseCol>(
        "SELECT *
    FROM base
    ORDER BY mid DESC
    LIMIT 1",
    )
    .fetch(conn);

    if let Ok(last) = rows.try_next().await {
        let mid = match last {
            Some(l) => Ok(l.mid),
            None => Ok(1),
        };

        return mid;
    }

    Ok(1)
}
