use crate::common::components::post::post_preview::PostPreviewParams;

pub async fn get_fake_post_preview_data() -> Vec<PostPreviewParams> {
    vec![
        PostPreviewParams {
            id: 1,
            title: "Rust is nice".to_string(),
            content: "I like Rust".to_string()
        }
    ]
}