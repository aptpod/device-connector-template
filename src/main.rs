mod hello;
mod hexdump;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use device_connector::conf::Conf;
use device_connector::{ElementBank, LoadedPlugin, RunnerBuilder};

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    config: PathBuf,
}

fn main() -> Result<()> {
    // Logging initialize
    env_logger::init();

    // Load config file
    let opts: Opts = Opts::parse();
    let conf = Conf::read_from_file(&opts.config)?;

    // Load element bank
    let mut bank = ElementBank::new();

    // Load plugins
    let loaded_plugin = LoadedPlugin::from_conf(&conf.plugin)?;
    loaded_plugin.load_plugins(&mut bank)?;

    // Append user defined Element
    bank.append_from_buildable::<hello::HelloSrcElement>()?;
    bank.append_from_buildable::<hexdump::HexdumpSinkElement>()?;

    // Start bg processes
    device_connector::process::start_bg_processes(&conf.bg_processes)?;

    // Execute before script and blocks until completion
    device_connector::process::exec_before_script(&conf.before_task)?;

    // Regester after script
    device_connector::process::register_after_script(&conf.after_task);

    // Create pipeline runner
    let mut runner_builder = RunnerBuilder::new(&bank, &loaded_plugin, &conf);
    runner_builder.append_from_conf(&conf.tasks)?;
    let runner = runner_builder.build()?;

    // Run pipeline
    runner.run()?;

    Ok(())
}
