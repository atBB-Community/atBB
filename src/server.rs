//! The application's web server.
//! Serves both api and web traffic.

use crate::{
    api::{self, ApiState, ApiStateInner},
    models::forum::{CategoryForum, Forum, ForumId},
    resources::DatabaseClient,
    web::{pages, WebState, WebStateInner},
    Config,
};
use axum::{Extension, Router};

pub async fn run(config: Config) -> color_eyre::Result<Router> {
    let database = DatabaseClient::new(config.database_kind, config.database_url).await?;
    database.migrate().await?;
    let database_health_check = health::PeriodicChecker::new(database, health::Config::default());
    tokio::spawn(database_health_check.clone().run());

    let web_state = WebState {
        inner: WebStateInner {
            forums: vec![CategoryForum {
                id: ForumId("1".into()),
                name: "Meta".to_string(),
                description: "Announcements and off topic discussion.".to_string(),
                forums: vec![
                    Forum {
                        id: ForumId("1-1".into()),
                        name: "Announcements".to_string(),
                        description: "Announcements and updates from the team.".to_string(),
                    },
                    Forum {
                        id: ForumId("1-2".into()),
                        name: "General".to_string(),
                        description: "General discussion that doesn't otherwise have a place.".to_string(),
                    },
                    Forum {
                        id: ForumId("1-3".into()),
                        name: "Off Topic".to_string(),
                        description: "Off topic discussion.".to_string(),
                    },
                ],
            },
            CategoryForum {
                id: ForumId("2".into()),
                name: "General".to_string(),
                description: "General discussion.".to_string(),
                forums: vec![
                    Forum {
                        id: ForumId("2-1".into()),
                        name: "Protocol".to_string(),
                        description: "Discussion about AT Protocol and its design.".to_string(),
                    },
                    Forum {
                        id: ForumId("2-2".into()),
                        name: "Accessibility".to_string(),
                        description: "Discussion about accessibility.".to_string(),
                    },
                    Forum {
                        id: ForumId("2-3".into()),
                        name: "Jetstream".to_string(),
                        description: "Jetstream allows for simpler consumption of the firehose.".to_string(),
                    },
                    Forum {
                        id: ForumId("2-4".into()),
                        name: "Custom Feeds".to_string(),
                        description: "Development and maintenance of custom feeds.".to_string(),
                    },

                    Forum {
                        id: ForumId("2-5".into()),
                        name: "Jetstream".to_string(),
                        description: "Jetstream allows for simpler consumption of the firehose.".to_string(),
                    },
                ],
            },
            CategoryForum {
                id: ForumId("3".into()),
                name: "Showcase".to_string(),
                description: "Showcase your projects and get feedback.".to_string(),
                forums: vec![
                    Forum {
                        id: ForumId("3-1".into()),
                        name: "Show & Tell".to_string(),
                        description: "Show off your projects and get feedback.".to_string(),
                    },
                ],
            },
            CategoryForum {
                id: ForumId("4".into()),
                name: "Programming Languages".to_string(),
                description: "Libraries and language support.".to_string(),
                forums: vec![
                    Forum {
                        id: ForumId("4-1".into()),
                        name: "Javascript".to_string(),
                        description: "Discussion about Javascript and its libraries.".to_string(),
                    },
                    Forum {
                        id: ForumId("4-2".into()),
                        name: "Rust".to_string(),
                        description: "Discussion about Rust and its libraries.".to_string(),
                    },
                    Forum {
                        id: ForumId("4-3".into()),
                        name: "Python".to_string(),
                        description: "Discussion about Python and its libraries.".to_string(),
                    },
                ],
            },
            CategoryForum {
                id: ForumId("5".into()),
                name: "Projects".to_string(),
                description: "Focused discussions about specific projects.".to_string(),
                forums: vec![
                    Forum {
                        id: ForumId("5-1".into()),
                        name: "frontpage.fyi".to_string(),
                        description: "https://frontpage.fyi".to_string(),
                    },
                    Forum {
                        id: ForumId("5-2".into()),
                        name: "atBB".to_string(),
                        description: "Discussion about atBB.".to_string(),
                    },
                ],
            }
            ],
        }
        .into(),
    };

    let api_state = ApiState {
        inner: ApiStateInner {
            forums: vec![CategoryForum {
                id: ForumId("1".into()),
                name: "Meta".to_string(),
                description: "Announcements and off topic discussion".to_string(),
                forums: vec![
                    Forum {
                        id: ForumId("1-1".into()),
                        name: "Announcements".to_string(),
                        description: "Announcements and updates from the team".to_string(),
                    },
                ],
            }],
        }
        .into(),
    };

    let router = Router::new()
        .nest("/", pages::compose(web_state))
        .nest("/api", api::routes::compose(api_state))
        .route_layer(Extension(database_health_check));
    Ok(router)
}
