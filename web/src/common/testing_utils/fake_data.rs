use crate::common::components::post::post_preview::PostPreviewParams;

pub async fn get_fake_post_preview_data() -> Vec<PostPreviewParams> {
    vec![
        PostPreviewParams {
            id: 1,
            title: "Rust is hard to learn".to_string(),
            content: r#"## Let's learn Rust
            *Here* is a super*duper*list of todo ***items***
            [This is the link name](https://helloworld.com)
            You can find more info here! [Go Here](https://gohere.com) click that link

            1. Buy milk
            2. Walk dog
            3. Run
            "#.to_string()
        },
        PostPreviewParams {
            id: 1,
            title: "Rust is not hard to Learn".to_string(),
            content: r"## Rust is not hard
            Here's a **list** of **items to first learn**, this is**super**duper **great fun**

            - Memory ownership
            - Async
            - Locking
            
            ".to_string()
        },
        PostPreviewParams {
            id: 1,
            title: "Rust coding".to_string(),
            content: r#"## Here's some code ***you*** might enjoy
            ![What a wonderful sight](http://sights.com)

            `fn go() -> String {
                // do work
                "hello world".to_string();
            }`
            "#.to_string()
        }
    ]
}