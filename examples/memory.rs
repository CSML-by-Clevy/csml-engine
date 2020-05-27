// use csmlinterpreter::data::csml_bot::CsmlBot;
// use csmlinterpreter::data::csml_flow::CsmlFlow;
// use csmlinterpreter::data::event::Event;
// use csmlinterpreter::data::ContextJson;
// use csmlinterpreter::interpret;
// use csmlinterpreter::validate_bot;

// const DEFAULT_ID_NAME: &str = "id";
// const DEFAULT_FLOW_NAME: &str = "default";
// const DEFAULT_STEP_NAME: &str = "start";
// const DEFAULT_BOT_NAME: &str = "my_bot";

// ////////////////////////////////////////////////////////////////////////////////
// // PUBLIC FUNCTION
// ////////////////////////////////////////////////////////////////////////////////

fn main() {}
//     let default_content = std::fs::read_to_string("CSML/examples/memory.csml").unwrap();
//     let default_flow = CsmlFlow::new(DEFAULT_ID_NAME, "default", &default_content, Vec::default());

//     // Create a CsmlBot
//     let bot = CsmlBot::new(
//         DEFAULT_ID_NAME,
//         DEFAULT_BOT_NAME,
//         None,
//         vec![default_flow],
//         DEFAULT_FLOW_NAME,
//     );
//     Value::Object(message)
// }

// fn interpret_flow(flow: &str) {
//     let event = Event::text("hello");
//     let mut metadata = Map::new();
//     let mut memories = Map::new();

//     metadata.insert("firstname".to_owned(), json!("Toto"));
//     metadata.insert("email".to_owned(), json!("toto@clevy.com"));

//     memories.insert(
//         "tmp".to_owned(),
//         serde_json::json!({
//             "_content": {
//                 "cards":[
//                     {
//                         "_content":{
//                             "buttons":[{"_content":{"accepts":["b1"],"icon":"info","payload":"b1","theme":"primary","title":"b1","toto":{"test":"plop"}},"_content_type":"button"}],
//                             "title":"c1"
//                         },
//                         "_content_type":"card"
//                     }
//                 ]
//             },
//             "_content_type":"carousel"
//         })
//     );

//     let mut context = ContextJson {
//         current: serde_json::json!(memories),
//         metadata: json!(metadata),
//         api_info: None,
//         hold: None,
//     };
//     let mut step = "start".to_owned();
//     let mut memory = Map::new();

//     while step != "end" {
//         let messages = interpret(flow, &step, context.clone(), &event, None);
//         dbg!(format_message(&messages));

//     // Create an Event
//     let event = Event::default();

//     // Create context
//     let context = ContextJson::new(
//         serde_json::json!({}),
//         serde_json::json!({}),
//         None,
//         None,
//         DEFAULT_STEP_NAME,
//         DEFAULT_FLOW_NAME,
//     );

//     // Run interpreter
//     let result = validate_bot(bot.to_owned());

//     if result.errors.is_some() {
//         dbg!(result.errors);
//         return;
//     }
//     if result.warnings.is_some() {
//         dbg!(result.warnings);
//     }

//     dbg!(interpret(bot, context, event, None));
// }

// // memoriers.insert(
// //     "tmp".to_owned(),
// //     serde_json::json!({
// //         "_content": {
// //             "cards":[
// //                 {
// //                     "_content":{
// //                         "buttons":[{"_content":{"accepts":["b1"],"icon":"info","payload":"b1","theme":"primary","title":"b1","toto":{"test":"plop"}},"_content_type":"button"}],
// //                         "title":"c1"
// //                     },
// //                     "_content_type":"card"
// //                 }
// //             ]
// //         },
// //         "_content_type":"carousel"
// //     })
// // );
