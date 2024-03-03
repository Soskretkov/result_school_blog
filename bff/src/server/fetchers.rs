mod protected;
use super::types::db_interaction::{Comment, Post as DbPost, User};
use crate::server::types::export::Post;
use crate::store;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    let path_suffix = format!("users/?login={}", &login);
    store::fetch::<Vec<User>>(&path_suffix)
        .await
        .map(|users_vec| users_vec.into_iter().next().map(|user| user.id))
}

pub async fn fetch_post(post_id: &str) -> Result<Post, String> {
    let post_path_suffix = format!("posts/{post_id}");
    let db_post = store::fetch::<DbPost>(&post_path_suffix).await?;

    let comments_path_suffix = format!("comments/?post_id={}", post_id);
    let comments = store::fetch::<Vec<Comment>>(&comments_path_suffix).await?;

    Ok(Post {
        id: db_post.id,
        title: db_post.payload.title,
        image_url: db_post.payload.image_url,
        content: db_post.payload.content,
        created_at: db_post.payload.created_at,
        comments,
    })
}
