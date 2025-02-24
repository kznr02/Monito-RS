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
    ColorTemp { value: u16 },
}

#[derive(Subcommand, Debug)]
enum GetCommand {
    Brightness,
    ColorTemp,
}

fn main() {
    let args = CliArgs::parse();

    env_logger::builder()
        .default_format()
        .format_file(true)
        .format_line_number(true)
        .init();

    // 枚举显示器实例数组
    let mut monitors = enumerate_monitor();

    match args.command {
        Commands::Set { monitor, command } => match command {
            SetCommand::Brightness { value } => {
                // 以指定的亮度设置与显示器编号设置显示器亮度，若无指定则统一调节所有显示器亮度
                if let Some(index) = monitor {
                    monitors[index as usize].set_monitor_brightness(value as u32);
                } else {
                    for m in monitors.iter_mut() {
                        m.set_monitor_brightness(value as u32);
                    }
                }
            }
            SetCommand::ColorTemp { value } => {
                if let Some(index) = monitor {
                    monitors[index as usize].set_monitor_temperature(value as u16);
                } else {
                    for m in monitors.iter_mut() {
                        m.set_monitor_temperature(value as u16);
                    }
                }
            }
        },
        Commands::Get { monitor, command } => {
            let mut ret = 0;
            match command {
                // 以指定的显示器编号获取显示器亮度，若无指定则统一获取所有显示器亮度
                GetCommand::Brightness => {
                    if let Some(index) = monitor {
                        ret = monitors[index as usize].get_monitor_brightness();
                        println!("Monitor {} Brightness: {}", index, ret);
                    } else {
                        for i in 0..monitors.len() {
                            ret = monitors[i].get_monitor_brightness();
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
            // 打印所有在位的物理显示器
            println!("Monitors:");
            for i in 0..monitors.len() {
                println!("Monitor {}: {}", i, monitors[i].name);
            }
        }
    }
}
