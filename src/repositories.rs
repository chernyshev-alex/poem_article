use anyhow::Result;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, QueryOrder, Set};
use std::sync::Arc;

use crate::domain::*;
use async_trait::async_trait;

pub struct ArticleCreate {
    pub title: String,
}

pub struct AuthorCreate {
    first_name: String,
    last_name: String,
    email: String,
}

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleRepositoryTrait: Sync + Send {
    async fn create(&self, f: &ArticleCreate) -> Result<article::ActiveModel>;
    async fn find_by_id(&self, id: i32) -> Result<Option<article::Model>>;
    async fn find_pages(&self, page: i32, page_size: i32) -> Result<Vec<article::Model>>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthorRepositoryTrait: Sync + Send {
    async fn create(&self, f: &AuthorCreate) -> Result<author::ActiveModel>;
    async fn get_by_id(&self, id: i32) -> Result<Option<author::Model>>;
}

#[async_trait]
pub trait Repository:
    ArticleRepositoryTrait + AuthorRepositoryTrait + Sync + Send + std::fmt::Debug
{
}

#[derive(Debug, Clone)]
pub struct DbRepository(Arc<DatabaseConnection>);

impl DbRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self(conn)
    }
}
impl Repository for DbRepository {}

#[async_trait]
impl ArticleRepositoryTrait for DbRepository {
    async fn create(&self, f: &ArticleCreate) -> Result<article::ActiveModel> {
        article::ActiveModel {
            title: Set(f.title.to_owned()),
            ..Default::default()
        }
        .save(self.0.as_ref())
        .await
        .map_err(Into::into)
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<article::Model>> {
        article::Entity::find_by_id(id)
            .one(self.0.as_ref())
            .await
            .map_err(|e| e.into())
    }
    async fn find_pages(&self, page: i32, page_size: i32) -> Result<Vec<article::Model>> {
        article::Entity::find()
            .order_by_asc(article::Column::Id)
            .paginate(self.0.as_ref(), page_size as u64)
            .fetch_page(page as u64)
            .await
            .map_err(Into::into)
    }
}

#[async_trait]
impl AuthorRepositoryTrait for DbRepository {
    async fn create(&self, f: &AuthorCreate) -> Result<author::ActiveModel> {
        author::ActiveModel {
            first_name: Set(f.first_name.to_owned()),
            last_name: Set(f.last_name.to_owned()),
            email: Set(f.email.to_owned()),
            ..Default::default()
        }
        .save(self.0.as_ref())
        .await
        .map_err(Into::into)
    }
    async fn get_by_id(&self, id: i32) -> Result<Option<author::Model>> {
        author::Entity::find_by_id(id)
            .one(self.0.as_ref())
            .await
            .map_err(Into::into)
    }
}

#[cfg(test)]
pub mod tests {
    use super::{
        ArticleCreate, ArticleRepositoryTrait, AuthorCreate, AuthorRepositoryTrait,
        MockArticleRepositoryTrait, MockAuthorRepositoryTrait, Repository,
    };
    use crate::domain::{article, author};
    use anyhow::Result;
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct MockRepository {
        article_repo: MockArticleRepositoryTrait,
        author_repo: MockAuthorRepositoryTrait,
    }
    impl MockRepository {
        pub fn new(
            article_repo: MockArticleRepositoryTrait,
            author_repo: MockAuthorRepositoryTrait,
        ) -> Self {
            Self {
                article_repo,
                author_repo,
            }
        }
    }
    impl Repository for MockRepository {}

    impl AuthorRepositoryTrait for MockRepository {
        fn create<'a, 'b, 'c>(
            &'a self,
            f: &'b AuthorCreate,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<author::ActiveModel>>
                    + ::core::marker::Send
                    + 'c,
            >,
        >
        where
            'a: 'c,
            'b: 'c,
        {
            self.author_repo.create(f)
        }

        fn get_by_id<'a, 'b>(
            &'a self,
            id: i32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Option<author::Model>>>
                    + ::core::marker::Send
                    + 'b,
            >,
        >
        where
            'a: 'b,
        {
            self.author_repo.get_by_id(id)
        }
    }

    impl ArticleRepositoryTrait for MockRepository {
        fn find_by_id<'a, 'b>(
            &'a self,
            id: i32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Option<article::Model>>>
                    + ::core::marker::Send
                    + 'b,
            >,
        >
        where
            'a: 'b,
        {
            self.article_repo.find_by_id(id)
        }

        fn find_pages<'a, 'b>(
            &'a self,
            page: i32,
            page_size: i32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Vec<article::Model>>>
                    + ::core::marker::Send
                    + 'b,
            >,
        >
        where
            'a: 'b,
        {
            self.article_repo.find_pages(page, page_size)
        }

        fn create<'a, 'b, 'c>(
            &'a self,
            f: &'b ArticleCreate,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<article::ActiveModel>>
                    + ::core::marker::Send
                    + 'c,
            >,
        >
        where
            'a: 'c,
            'b: 'c,
        {
            self.article_repo.create(f)
        }
    }
}
