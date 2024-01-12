use sqlx::{Execute, QueryBuilder, Sqlite, SqliteConnection};

pub fn init_log() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
}

pub struct BaseCol {
    pub mid: i32,
    pub lable_theme: String,
    pub name: String,
}

pub async fn insert(conn: &mut SqliteConnection, data: &Vec<BaseCol>) -> anyhow::Result<()> {
    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO base (mid,lable_theme,name)");

    query_builder.push_values(data.iter().take(data.len()), |mut b, user| {
        b.push_bind(user.mid)
            .push_bind(user.lable_theme.clone())
            .push_bind(user.name.clone());
    });

    let query_sql = query_builder.build();
    sqlx::query(query_sql.sql()).execute(conn).await?;
    Ok(())
}
