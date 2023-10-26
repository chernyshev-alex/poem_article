use poem_article::domain::article;
use sea_orm::sea_query::ColumnDef;
use sea_orm::*;

#[tokio::test]
async fn test_main() -> Result<(), DbErr> {
    let conn: DatabaseConnection = Database::connect("sqlite::memory:").await?;
    setup_schema(&conn).await?;
    crud_article(&conn).await?;
    conn.close().await?;
    Ok(())
}

async fn setup_schema(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let stmt = sea_query::Table::create()
        .table(article::Entity)
        .col(
            ColumnDef::new(article::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(article::Column::Title).string())
        .col(ColumnDef::new(article::Column::Content).string().null())
        .to_owned();

    let builder = conn.get_database_backend();
    let result = conn.execute(builder.build(&stmt)).await?;
    println!("Created table: {result:?}");
    Ok(())
}

async fn crud_article(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let mut article = article::ActiveModel {
        title: Set("ATitle".to_owned()),
        ..Default::default()
    }
    .save(conn)
    .await?;

    assert_eq!(
        article,
        article::ActiveModel {
            id: Unchanged(1),
            title: Unchanged("ATitle".to_owned()),
            content: Unchanged(None),
        }
    );

    article.title = Set("BTitle".to_owned());
    let _ = article.save(conn).await?;
    let article = article::Entity::find_by_id(1).one(conn).await?;

    assert_eq!(
        article,
        Some(article::Model {
            id: 1,
            title: "BTitle".to_owned(),
            content: None,
        })
    );

    Ok(())
}
