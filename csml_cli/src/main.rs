mod init_package;
mod interface;
mod run;

use clap::{App, AppSettings, Arg, SubCommand};
use csml_engine::data::BotOpt;

use interface::{chat_menu::format_initial_payload, StartUI};
use run::load_info;

fn main() {
    let matches = App::new("CSML CLI")
        // .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommands(vec![
            SubCommand::with_name("run")
                .about("Run Bot")
                .arg(
                    Arg::with_name("text")
                        .short("t")
                        .long("text")
                        .value_name("TEXT")
                        .help("start run with text")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("flow")
                        .short("f")
                        .long("flow")
                        .value_name("FLOW")
                        .help("Select starting flow")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("step")
                        .short("s")
                        .long("step")
                        .value_name("STEP")
                        .help("Select starting step")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .long("debug")
                        .help("Print debug information's")
                        .takes_value(true),
                ),
            SubCommand::with_name("init").about("Create a new CSML Bot in the selected directory"),
        ])
        .get_matches();

    if let Some(sub_commands) = &matches.subcommand {
        match sub_commands.name.as_str() {
            "init" => interface::csml_ui(StartUI::Init).unwrap(),
            "run" => {
                if let Some(run) = matches.subcommand_matches("run") {
                    let flow = run.value_of("flow");
                    let step = run.value_of("step");
                    let text = run.value_of("text");

                    let request = format_initial_payload(flow, step, text);

                    match load_info(".") {
                        Ok(bot) => {
                            let bot_opt = Some(BotOpt::CsmlBot(bot));

                            let start = StartUI::Run { request, bot_opt };

                            interface::csml_ui(start).unwrap();
                        }
                        Err(..) => {
                            println!("path [./] is not a valid bot directory")
                        }
                    }
                }
            }
            _ => interface::csml_ui(StartUI::Main).unwrap(),
        }
    } else {
        interface::csml_ui(StartUI::Main).unwrap()
    }
}
