use crate::{
    domain::article,
    repositories::{ArticleCreate, ArticleRepositoryTrait, AuthorRepositoryTrait, Repository},
};
use anyhow::Result;
use std::{fmt::Debug, sync::Arc};

#[cfg(test)]
use mockall::{automock, predicate::*};
use poem::async_trait;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleServiceTrait: Sync + Send + Debug {
    async fn create_article(&self, title: &str) -> Result<article::ActiveModel>;
    async fn list_articles(&self, num_page: i32, page_size: i32) -> Result<Vec<article::Model>>;
    async fn get_article_by_id(&self, id: i32) -> Result<Option<article::Model>>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SocialMediaPublisherTrait: Sync + Send + Debug {
    async fn publish_article(&self, article: &article::ActiveModel) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct ArticleServiceSt {
    pub repo: Arc<dyn Repository>,
}

impl ArticleServiceSt {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl ArticleServiceTrait for ArticleServiceSt {
    async fn create_article(&self, title: &str) -> Result<article::ActiveModel> {
        let msg = ArticleCreate {
            title: title.to_string(),
        };
        ArticleRepositoryTrait::create(self.repo.as_ref(), &msg).await
    }

    async fn list_articles(&self, page: i32, page_size: i32) -> Result<Vec<article::Model>> {
        ArticleRepositoryTrait::find_pages(self.repo.as_ref(), page, page_size).await
    }

    async fn get_article_by_id(&self, id: i32) -> Result<Option<article::Model>> {
        ArticleRepositoryTrait::find_by_id(self.repo.as_ref(), id).await
    }
}

#[derive(Debug, Clone)]
pub struct SocialMediaPublisher;

#[async_trait]
impl SocialMediaPublisherTrait for SocialMediaPublisher {
    async fn publish_article(&self, _article: &article::ActiveModel) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        domain::article,
        repositories::{
            tests::MockRepository, MockArticleRepositoryTrait, MockAuthorRepositoryTrait,
        },
        services::{ArticleServiceSt, ArticleServiceTrait},
    };
    use mockall::predicate;
    use sea_orm::{Set, Unchanged};
    use std::sync::Arc;

    fn mocked_service(
        mock_article: MockArticleRepositoryTrait,
        mock_author: MockAuthorRepositoryTrait,
    ) -> ArticleServiceSt {
        let mock_repo = MockRepository::new(mock_article, mock_author);
        ArticleServiceSt::new(Arc::new(mock_repo))
    }

    #[tokio::test]
    async fn get_by_id() {
        let mock_author = MockAuthorRepositoryTrait::new();
        let mut mock_article = MockArticleRepositoryTrait::new();
        mock_article
            .expect_find_by_id()
            .with(predicate::ge(0))
            .returning(|id| {
                Ok(Some(article::Model {
                    id,
                    title: "title".to_string(),
                    content: None,
                }))
            });

        let service = mocked_service(mock_article, mock_author);
        let result = service.get_article_by_id(1).await;
        assert!(result.is_ok() && result.unwrap().unwrap().id == 1);
    }

    #[tokio::test]
    async fn create_article() {
        let mock_author = MockAuthorRepositoryTrait::new();
        let mut mock_article = MockArticleRepositoryTrait::new();
        mock_article.expect_create().returning(|ac| {
            Ok(article::ActiveModel {
                id: Set(1),
                title: Set(ac.title.clone()),
                content: Unchanged(None),
            })
        });

        let service = mocked_service(mock_article, mock_author);
        let result = service.create_article("article").await;
        assert!(result.is_ok() && result.unwrap().title.unwrap() == "article");
    }

    #[tokio::test]
    async fn find_pages() {
        let mock_author = MockAuthorRepositoryTrait::new();
        let mut mock_article = MockArticleRepositoryTrait::new();
        mock_article
            .expect_find_pages()
            .returning(|_page, _page_size| {
                Ok(vec![article::Model {
                    id: 1,
                    title: "article1".to_string(),
                    content: None,
                }])
            });

        let service = mocked_service(mock_article, mock_author);
        let result = service.list_articles(0, 10).await;
        assert!(result.is_ok() && result.unwrap().len() > 0);
    }
}
