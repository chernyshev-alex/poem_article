use poem::error::{InternalServerError, NotFoundError};
use poem::http::StatusCode;
use poem::middleware::{TokioMetrics, Tracing};
use poem::web::{Data, Form, Html, Json, Path, Query};
use poem::{get, handler, post, Endpoint, EndpointExt, IntoResponse, Response, Result, Route};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::domain::article;
use crate::AppStateM;

#[derive(Deserialize, Serialize)]
pub struct Params {
    page: i32,
    page_size: i32,
}

#[handler]
pub async fn create_article(
    state: Data<&AppStateM>,
    form: Form<article::Model>,
) -> Result<impl IntoResponse> {
    let result = state.service.create_article(form.title.as_str()).await;
    Ok(Json(
        result.map(article::Model::from).map_err(|e| e.to_string()),
    ))
}

#[handler]
pub async fn list_articles(
    state: Data<&AppStateM>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse> {
    let result = state
        .service
        .list_articles(params.page, params.page_size)
        .await;
    Ok(Json(result.map_err(|e| e.to_string())))
}

#[handler]
pub async fn get_article_by_id(
    state: Data<&AppStateM>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let result = state.service.get_article_by_id(id).await;
    Ok(Json(result.map_err(|e| e.to_string())))
}

#[handler]
pub async fn get_tokyo_metric() -> &'static str {
    "tokyo_metric"
}

#[handler]
fn index_view(state: Data<&AppStateM>) -> Result<impl IntoResponse> {
    let tera = &state.templates;
    tera.render("index.html.tera", &Context::new())
        .map_err(InternalServerError)
        .map(Html)
}

#[handler]
fn stats_view(state: Data<&AppStateM>) -> Result<impl IntoResponse> {
    let tera = &state.templates;
    tera.render("stats.html.tera", &Context::new())
        .map_err(InternalServerError)
        .map(Html)
}

#[handler]
fn articles_view(state: Data<&AppStateM>) -> Result<impl IntoResponse> {
    let tera = &state.templates;
    tera.render("article.html.tera", &Context::new())
        .map_err(InternalServerError)
        .map(Html)
}

pub fn config_router(state: AppStateM) -> impl Endpoint<Output = Response> {
    let tokyo_metric = TokioMetrics::new();
    let list_articles_metric = TokioMetrics::new();
    Route::new()
        .at("/", get(index_view))
        .at("/stats", get(stats_view))
        .at("/articles_view", get(articles_view))
        .at(
            "/articles",
            post(create_article)
                .get(list_articles)
                .with(list_articles_metric),
        )
        .at("/articles/:id", get(get_article_by_id))
        // .at("/new", new)
        // .at("/:id", get(edit).post(update))
        // .nest(
        //     "/static",
        //     StaticFilesEndpoint::new(format!("{}/static", resources_path)),
        // )
        .at("/metrics/tokyo", tokyo_metric.exporter())
        .at("/tokyo", get(get_tokyo_metric))
        .with(tokyo_metric)
        .with(Tracing)
        .data(state)
        .catch_error(|_: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Error")
        })
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::domain::article;
    use mockall::predicate::*;
    use poem::{http::StatusCode, test::TestClient, Endpoint, Response};
    use sea_orm::Set;
    use tera::Tera;

    // fn get_client(
    //     state: AppState,
    //     mock: MockArticleServiceTrait,
    // ) -> TestClient<impl Endpoint<Output = Response>> {
    //     let mut mock_service = MokArticleServiceTr::new();
    //     let mock_app_state = AppStateM {
    //         service: Arc::new(mock_service),
    //         publisher: Arc::new(MockSocialMediaPublisherTrait {}),
    //         templates: Tera::new("./templates/**/*").unwrap(),
    //     };

    //     let router = create_router(mock_app_state);
    //     TestClient::new(router)
    // }

    // #[tokio::test]
    // async fn get_article_by_id() {
    //     let mut mock = MockArticleServiceTrait::new();
    //     mock.get_article_by_id(1)
    //         .await
    //         .with(always())
    //         .returning(|id| {
    //             Ok(Some(article::Model {
    //                 id,
    //                 title: "title".to_string(),
    //                 content: None,
    //             }))
    //         });

    //     let cli = get_client(AppState { templates: None }, mock);
    //     let resp = cli.get("/articles/1").send().await;
    //     resp.assert_status_is_ok();
    // }

    // #[tokio::test]
    // async fn create_article() {
    //     let mut mock = MockArticleServiceTrait::new();
    //     mock.create_article("title")
    //         .with(always())
    //         .returning(|_title| {
    //             Ok(article::ActiveModel {
    //                 id: Set(1),
    //                 title: Set(_title.to_string()),
    //                 content: Set(None),
    //             })
    //         });

    //     let cli = get_client(AppState { templates: None }, mock);
    //     let resp = cli.post("/articles?title=artcile_title").send().await;
    //     resp.assert_status(StatusCode::CREATED);
    // }

    // #[tokio::test]
    // async fn list_articles() {
    //     let mut mock = MockArticleServiceTrait::new();
    //     mock.list_articles(0, 20)
    //         .with(always(), always())
    //         .returning(|page, page_size| {
    //             Ok(vec![article::Model {
    //                 id: 1,
    //                 title: "tille".to_string(),
    //                 content: None,
    //             }])
    //         });

    //     let cli = get_client(AppState { templates: None }, mock);
    //     let resp = cli.get("/articles?page=0&page_size=20").send().await;
    //     resp.assert_status_is_ok();
    // }
}
