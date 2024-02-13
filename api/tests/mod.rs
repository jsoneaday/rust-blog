pub mod common {
    pub mod authentication {
        pub mod test_auth_service;
    }
    pub mod repository {
        pub mod administrator {
            pub mod test_administrator;
        }
        pub mod post {
            pub mod test_post;
        }
        pub mod mail {
            pub mod test_mail;
        }
    }    
}
pub mod routes {
    pub mod authentication {
        pub mod test_authentication_routes;
    }
    pub mod post {
        pub mod test_post_routes;
    }
    pub mod mail {
        pub mod test_mail_routes;
    }
}