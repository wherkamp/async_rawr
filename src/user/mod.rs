use log::trace;
use crate::me::Me;
use crate::responses::comments::Comments;
use crate::responses::submission::Submissions;
use crate::responses::user::UserResponse;
use crate::responses::{GenericListing, RedditListing, RedditType};
use crate::utils::error::APIError;
use crate::utils::options::FeedOption;

/// The User Object for Reddit
pub struct User<'a> {
    pub(crate) me: &'a Me,
    pub name: String,
}

impl<'a> PartialEq for User<'a> {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name
    }
}

impl<'a> User<'a> {
    /// Gets the about data for the user
    pub async fn about(&self) -> Result<UserResponse, APIError> {
        let url = format!("/user/{}/about.json", &self.name);
        let response = self.me.get(&url, false).await?;
        if !response.status().is_success() {
            trace!("Bad Response Status {}", response.status().as_u16() );
            return Err(response.status().clone().into());
        }
        let value = response.text().await?;
        trace!("{}",&value);
        let x: UserResponse = serde_json::from_str(value.as_str())?;
        return Ok(x);
    }
    /// Comments
    pub async fn comments(&self, feed: Option<FeedOption>) -> Result<Comments, APIError> {
        let mut string = format!("/user/{}/comments.json", &self.name);
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Comments>(&*string, false).await;
    }
    /// user Submissions
    pub async fn submissions(&self, feed: Option<FeedOption>) -> Result<Submissions, APIError> {
        let mut string = format!("/user/{}/submitted.json", &self.name);
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Submissions>(&*string, false).await;
    }
    /// User Overview
    pub async fn overview(&self, feed: Option<FeedOption>) -> Result<RedditListing, APIError> {
        let mut string = format!("/user/{}/overview.json", &self.name);
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<RedditListing>(&*string, false).await;
    }
    /// Get User saved post. The user must be logged in
    pub async fn saved(&self, feed: Option<FeedOption>) -> Result<RedditListing, APIError> {
        let mut string = format!("/user/{}/saved.json", &self.name);
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<RedditListing>(&*string, true).await;
    }
}
