// TODO: read amine.toml
// TODO: proper error management
// TODO: this code is dogshit, rewrite it

use std::error::Error;
use std::path::Path;
use emulator::cpu::CPU;
use emulator::device::{Device, Display, Dummy};
use emulator::plugin::{ClearDbg, Corruption, LimitClockSpeed, Plugin, PrintDbg, RamView, RegisterInsight, StructInsight};
use toml::{Table, Value};
use crate::checksum;
use crate::config::ConfigError::UnknownPlugin;

pub struct AmineConfig {
    path: String,
    code_path: String,
    updates_per_cycle: usize,
    devices: Vec<Box<dyn Device>>,
    plugins: Vec<Box<dyn Plugin>>,
}

fn construct_device(name: &str) -> Option<Box<dyn Device>> {
    //TODO: allow { name = "...", context = "..." }
    match name {
        "display" => Some(Box::new(Display::new())),
        _ => None,
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingPluginParameter { plugin: String, parameter: String },
    InvalidPluginParameters { plugin: String },
    InvalidParameterType { plugin: String, parameter: String },
    UnknownPlugin { plugin: String },
    InvalidPort { port: String },
    InvalidDevice { device: String },
}

impl ConfigError {
    fn missing_plugin_parameter(plugin: impl Into<String>, parameter: impl Into<String>) -> Self {
        ConfigError::MissingPluginParameter {
            plugin: plugin.into(),
            parameter: parameter.into(),
        }
    }

    fn invalid_parameter_type(plugin: impl Into<String>, parameter: impl Into<String>) -> Self {
        ConfigError::MissingPluginParameter {
            plugin: plugin.into(),
            parameter: parameter.into(),
        }
    }
}

macro_rules! get_param {
    ($config:expr, $parent:expr, $key:expr, $t:pat, $i:expr, $default:expr) => {
        match $config.get($key) {
            Some($t) => Ok($i),
            Some(_) => Err(ConfigError::invalid_parameter_type($parent, $key)),
            None => Ok($default),
        }
    };
    ($config:expr, $parent:expr, $key:expr, $t:pat, $i:expr) => {
        match $config.get($key) {
            Some($t) => Ok($i),
            Some(_) => Err(ConfigError::invalid_parameter_type($parent, $key)),
            None => Err(ConfigError::missing_plugin_parameter($parent, $key)),
        }
    };
}

fn construct_plugin(plugin: &str, config: &Table) -> Result<Box<dyn Plugin>, ConfigError> {
    match plugin {
        "clear-dbg" => Ok(Box::new(ClearDbg)),
        "corruption" => {
            let instructions_per_flip = *get_param!(config, "corruption", "instructions-per-flip", Value::Integer(i), i)? as usize;
            Ok(Box::new(Corruption::new(instructions_per_flip)))
        },
        "limit-clock-speed" => {
            let ips = *get_param!(config, "limit-clock-speed", "ips", Value::Integer(i), i)? as usize;
            Ok(Box::new(LimitClockSpeed::new(ips)))
        },
        "print-dbg" => {
            Ok(Box::new(PrintDbg))
        },
        "ram-view" => Ok(Box::new(RamView::new().unwrap())),
        "register-insight" => {
            let show_next_instruction = *get_param!(config, "register-insight", "show-next-instruction", Value::Boolean(b), b, &false)?;
            let show_stack_window = *get_param!(config, "register-insight", "show-stack-window", Value::Boolean(b), b, &false)?;
            let show_ram_checksum = *get_param!(config, "register-insight", "show-ram-checksum", Value::Boolean(b), b, &false)?;
            Ok(Box::new(RegisterInsight::new(show_next_instruction, show_stack_window, show_ram_checksum)))
        },
        "struct-insight" => {
            let registers = get_param!(config, "struct-insight", "registers", Value::Array(a), a.clone())?;
            let registers = [
                registers.contains(&Value::String("r0".to_owned())),
                registers.contains(&Value::String("r1".to_owned())),
                registers.contains(&Value::String("r2".to_owned())),
                registers.contains(&Value::String("r3".to_owned())),
                registers.contains(&Value::String("r4".to_owned())),
                registers.contains(&Value::String("r5".to_owned())),
                registers.contains(&Value::String("r6".to_owned())),
                registers.contains(&Value::String("r7".to_owned())),
            ];
            let depth = *get_param!(config, "struct-insight", "depth", Value::Integer(i), i)? as usize;
            Ok(Box::new(StructInsight::new(registers, depth)))
        },
        _ => Err(UnknownPlugin { plugin: plugin.to_string() }),
    }
}

impl AmineConfig {
    pub fn import(path: &str) -> Result<AmineConfig, ConfigError> {
        let table: Table = std::fs::read_to_string(Path::new(path).join("amine.toml"))
            .unwrap()
            .parse()
            .unwrap();

        // total dog shit
        let code_path = table.get("runtime").unwrap().as_table().unwrap().get("src-path").unwrap().as_str().unwrap().to_string();
        let updates_per_cycle = table.get("runtime").unwrap().as_table().unwrap().get("updates-per-cycle").unwrap().as_integer().unwrap() as usize;

        let mut devices: Vec<Box<dyn Device>> = Vec::new();
        for (port, device) in get_param!(table, "", "devices", Value::Table(t), t)? {
            let Ok(port) = port.parse::<u16>() else {
                return Err(ConfigError::InvalidPort { port: port.to_string() });
            };
            let Value::String(device) = device else {
                return Err(ConfigError::InvalidDevice { device: device.to_string() });
            };
            let Some(device) = construct_device(&device) else {
                return Err(ConfigError::InvalidDevice { device: device.to_string() });
            };
            for _dummy in devices.len()..port as usize + 1 {
                devices.push(Box::new(Dummy))
            }
            devices[port as usize] = device;
        }

        let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
        for (plugin, table) in get_param!(table, "", "plugins", Value::Table(t), t)? {
            let Value::Table(table) = table else {
                return Err(ConfigError::InvalidPluginParameters { plugin: plugin.to_string() });
            };
            let plugin = construct_plugin(plugin, &table)?;
            plugins.push(plugin);
        }

        Ok(AmineConfig {
            path: path.to_string(),
            code_path,
            updates_per_cycle,
            devices,
            plugins,
        })
    }

    pub fn create(self) -> (CPU, usize) {
        let path = Path::new(&self.path).join(self.code_path);
        let bytecode = assembler::assemble_project(&path).unwrap();
        let len = bytecode
            .iter()
            .enumerate()
            .fold(0, |result, (idx, word)| match word {
                0 => result,
                _ => idx + 1,
            });
        println!("{0:?}", &bytecode[..len]);
        println!("word count: {len} ({0:.2}% of RAM)", len as f32 / 655.36);
        println!("checksum: {}", checksum(bytecode.as_slice()));
        let mut cpu = CPU::from(bytecode);
        for device in self.devices {
            cpu.attach(device);
        }
        for plugin in self.plugins {
            cpu.install(plugin);
        }
        (cpu, self.updates_per_cycle)
    }
}