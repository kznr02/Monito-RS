use clap::{Parser, Subcommand};
use monito_lib::*;

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct CliArgs {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Set {
        #[clap(value_name = "monitor", short, long)]
        monitor: Option<u8>,
        #[clap(subcommand)]
        command: SetCommand,
    },
    #[clap(arg_required_else_help = true)]
    Get {
        #[clap(value_name = "monitor", short, long)]
        monitor: Option<u8>,
        #[clap(subcommand)]
        command: GetCommand,
    },
    // #[clap(arg_required_else_help = true)]
    List,
}

#[derive(Subcommand, Debug)]
enum SetCommand {
    Brightness { value: u8 },
    ColorTemp { value: u8 },
}

#[derive(Subcommand, Debug)]
enum GetCommand {
    Brightness,
    ColorTemp,
}

fn main() {
    let args = CliArgs::parse();
    let mut monitors = enumerate_monitor();

    match args.command {
        Commands::Set { monitor, command } => match command {
            SetCommand::Brightness { value } => {
                if let Some(index) = monitor {
                    set_monitor_brightness(&mut monitors[index as usize], value as u32);
                } else {
                    for m in monitors.iter_mut() {
                        set_monitor_brightness(m, value as u32);
                    }
                }
            }
            SetCommand::ColorTemp { value } => {
                todo!();
            }
        },
        Commands::Get { monitor, command } => {
            let mut ret = 0;
            match command {
                GetCommand::Brightness => {
                    if let Some(index) = monitor {
                        ret = get_monitor_brightness(&mut monitors[index as usize]);
                        println!("Monitor {} Brightness: {}", index, ret);
                    } else {
                        for i in 0..monitors.len() {
                            ret = get_monitor_brightness(&mut monitors[i]);
                            println!("Monitor {} Brightness: {}", i, ret);
                        }
                    }
                }
                GetCommand::ColorTemp => {
                    todo!();
                }
            }
        }
        Commands::List => {
            println!("Monitors:");
            for i in 0..monitors.len() {
                println!("Monitor {}: {}", i, monitors[i].name);
            }
        }
    }
}
