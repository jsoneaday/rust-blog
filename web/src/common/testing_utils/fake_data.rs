use crate::common::components::post::post_preview::PostPreviewParams;

pub async fn get_fake_post_preview_data() -> Vec<PostPreviewParams> {
    vec![
        PostPreviewParams {
            id: 1,
            title: "Rust is nice".to_string(),
            content: r"# Let's learn Rust
            Here is a list of todo items
            1. Buy milk
            2. Walk dog
            3. Run
            ".to_string()
        },
        PostPreviewParams {
            id: 1,
            title: "Rust is nice".to_string(),
            content: r"# Rust is not hard
            Here's a list of items to first learn
            - Memory ownership
            - Async
            - Locking
            
            ".to_string()
        },
        PostPreviewParams {
            id: 1,
            title: "Rust is nice".to_string(),
            content: "I like Rust".to_string()
        }
    ]
}