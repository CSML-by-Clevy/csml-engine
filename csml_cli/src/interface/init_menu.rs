use crate::interface::{MenuComponent, AppState, MenuItem, MenuType};
use std::iter::Iterator;

const AWS_REGIONS_PAYLOAD: &[&str] = &[
    "US East (Ohio) us-east-2",
    "US East (N. Virginia) us-east-1",
    "US West (N. California) us-west-1",
    "US West (Oregon) us-west-2",
    "Africa (Cape Town) af-south-1",
    "Asia Pacific (Hong Kong) ap-east-1",
    "Asia Pacific (Mumbai) ap-south-1",
    "Asia Pacific (Osaka) ap-northeast-3",
    "Asia Pacific (Seoul) ap-northeast-2",
    "Asia Pacific (Singapore) ap-southeast-1",
    "Asia Pacific (Sydney) ap-southeast-2",
    "Asia Pacific (Tokyo) ap-northeast-1",
    "Canada (Central) ca-central-1",
    "Europe (Frankfurt) eu-central-1",
    "Europe (Ireland) eu-west-1",
    "Europe (London) eu-west-2",
    "Europe (Milan) eu-south-1",
    "Europe (Paris) eu-west-3",
    "Europe (Stockholm) eu-north-1",
    "Middle East (Bahrain) me-south-1",
    "South America (São Paulo) sa-east-1",
    "AWS GovCloud (US-East) us-gov-east-1",
    "AWS GovCloud (US-West) us-gov-west-1",

    "localhost",
];

const AWS_REGIONS: &[&str] = &[
    "us-east-2",
    "us-east-1",
    "us-west-1",
    "us-west-2",
    "af-south-1",
    "ap-east-1",
    "ap-south-1",
    "ap-northeast-3",
    "ap-northeast-2",
    "ap-southeast-1",
    "ap-southeast-2",
    "ap-northeast-1",
    "ca-central-1",
    "eu-central-1",
    "eu-west-1",
    "eu-west-2",
    "eu-south-1",
    "eu-west-3",
    "eu-north-1",
    "me-south-1",
    "sa-east-1",
    "us-gov-east-1",
    "us-gov-west-1",

    "localhost",
];

fn get_aws_regions() -> Vec<MenuComponent> {
    AWS_REGIONS.iter().zip(AWS_REGIONS_PAYLOAD.iter()).map(|(&text, &payload)| {

        let mut sub_component = vec![];

        if payload == "localhost" {
            sub_component.push(
                MenuItem::new(
                    "Dynamodb port",
                    MenuComponent::Text{
                        text: "8000".to_owned(),
                        payload: None,
                        sub_component: vec![]
                    },
                    2
                )
            );
        }

        MenuComponent::Text{
            text: text.to_owned(),
            payload: Some(payload.to_owned()),
            sub_component
        }
    }).collect()
}

fn get_s3_regions() -> Vec<MenuComponent> {
    AWS_REGIONS.iter().zip(AWS_REGIONS_PAYLOAD.iter()).map(|(&text, &payload)| {

        let mut sub_component = vec![];

        if payload == "localhost" {
            sub_component.push(
                MenuItem::new(
                    "Localstack port",
                    MenuComponent::Text{
                        text: "8000".to_owned(), payload: None, sub_component: vec![]
                    },
                    2
                )
            )
        }

        MenuComponent::Text{
            text: text.to_owned(),
            payload: Some(payload.to_owned()),
            sub_component
        }
    }).collect()
}


#[derive(Debug, Clone)]
pub struct InitMenu {
    pub start: Vec<MenuItem>,
}

impl MenuType for InitMenu {
    fn init() -> Box<dyn MenuType> {
        let mongodb_sub_menu = vec![
            MenuItem::new(
                "MongoDB Connection String URI",
                MenuComponent::Text{text: "mongodb://localhost:27017".to_owned(), payload: None, sub_component: vec![]},
                1,
            )
        ];

        let postgres_sub_menu = vec![
            MenuItem::new(
                "POSTGRESQL Connection String URI",
                MenuComponent::Text{text: "postgres://localhost:27017".to_owned(), payload: None, sub_component: vec![]},
                1,
            )
        ];

        let dynamodb_sub_menu = vec![
            MenuItem::new(
                "DynamoDB region",
                MenuComponent::List {
                    components: get_aws_regions(),
                    selected: 0,
                    scroll_index: 0,
                },
                1,
            ),
            MenuItem::new(
                "Dynamodb table name",
                MenuComponent::Text{text: "csml-engine-db-local".to_owned(), payload: None, sub_component: vec![]},
                1,
            ),
            MenuItem::new(
                "S3 bucket Location",
                MenuComponent::List {
                    components: get_s3_regions(),
                    selected: 0,
                    scroll_index: 0,
                },
                1,
            ),
            MenuItem::new(
                "S3 bucket name",
                MenuComponent::Text{text: "csml-engine-bucket".to_owned(), payload: None, sub_component: vec![]},
                1,
            ),
        ];

        let init = InitMenu {
            start: vec![
                MenuItem::new("Chatbot name", MenuComponent::Text{text: "bot".to_owned(), payload: None, sub_component: vec![]}, 0),
                MenuItem::new(
                    "Enable variable encryption",
                    MenuComponent::List {
                        components: vec![
                                MenuComponent::Text{text: "yes".to_owned(), payload: None, sub_component: vec![]},
                                MenuComponent::Text{text: "no".to_owned(), payload: None, sub_component: vec![]}
                            ],
                        selected: 0,
                        scroll_index: 0,
                    },
                    0,
                ),
                MenuItem::new(
                    "Database type",
                    MenuComponent::List {
                        components: vec![
                                MenuComponent::Text{text: "mongodb".to_owned(), payload: None, sub_component: mongodb_sub_menu },
                                MenuComponent::Text{text: "dynamodb".to_owned(), payload: None, sub_component: dynamodb_sub_menu},
                                MenuComponent::Text{text: "postgresql".to_owned(), payload: None, sub_component: postgres_sub_menu}
                            ],
                        selected: 0,
                        scroll_index: 0,
                    },
                    0,
                ),
                MenuItem::new(
                    "",
                    MenuComponent::Button{text: "               [Validate]".to_owned(), payload: None},
                    0,
                ),
            ],
        };

        Box::new(init)
    }

    fn get_default_state() -> AppState {
        AppState::Normal
    }

    fn generate_menu(&mut self) -> Vec<MenuItem> {
        let mut new_menu = vec![];

        new_menu.append(&mut self.start.clone());

        new_menu
    }
}