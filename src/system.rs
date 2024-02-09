use std::sync::{Arc, Mutex};

use crate::{
    constants::{MAX_CORE_SUBSYSTEM_INFO, MAX_CORE_SUBSYSTEM_ROM_INFO},
    controller_info::ControllerInfo,
    core::RetroContext,
    libretro::binding_libretro::{
        retro_subsystem_info, retro_subsystem_memory_info, retro_subsystem_rom_info,
        retro_system_info, LibretroRaw,
    },
    tools::{ffi_tools::get_str_from_ptr, mutex_tools::get_string_mutex_from_ptr},
};

#[derive(Default)]
pub struct SysInfo {
    pub library_name: Mutex<String>,
    pub library_version: Mutex<String>,
    pub valid_extensions: Mutex<String>,
    pub need_fullpath: Mutex<bool>,
    pub block_extract: Mutex<bool>,
}

#[derive(Default)]
pub struct MemoryInfo {
    pub extension: Mutex<String>,
    pub type_: Mutex<u32>,
}

#[derive(Default)]
pub struct SubSystemRomInfo {
    pub desc: Mutex<String>,
    pub valid_extensions: Mutex<String>,
    pub need_fullpath: Mutex<bool>,
    pub block_extract: Mutex<bool>,
    pub required: Mutex<bool>,
    pub memory: MemoryInfo,
    pub num_memory: Mutex<u32>,
}

#[derive(Default)]
pub struct SubSystemInfo {
    pub id: Mutex<u32>,
    pub desc: Mutex<String>,
    pub ident: Mutex<String>,
    pub roms: Mutex<Vec<SubSystemRomInfo>>,
}

#[derive(Default)]
pub struct System {
    pub ports: Mutex<Vec<ControllerInfo>>,
    pub info: SysInfo,
    pub subsystem: Mutex<Vec<SubSystemInfo>>,
}

pub fn get_sys_info(raw: &LibretroRaw) -> SysInfo {
    unsafe {
        let sys_info = &mut retro_system_info {
            block_extract: false,
            need_fullpath: false,
            library_name: "".as_ptr() as *const i8,
            library_version: "".as_ptr() as *const i8,
            valid_extensions: "".as_ptr() as *const i8,
        };

        raw.retro_get_system_info(sys_info);

        SysInfo {
            library_name: Mutex::new(get_str_from_ptr(sys_info.library_name)),
            library_version: Mutex::new(get_str_from_ptr(sys_info.library_version)),
            valid_extensions: Mutex::new(get_str_from_ptr(sys_info.valid_extensions)),
            need_fullpath: Mutex::new(sys_info.need_fullpath),
            block_extract: Mutex::new(sys_info.block_extract),
        }
    }
}

pub fn get_subsystem(
    ctx: &Arc<RetroContext>,
    raw_subsystem: [retro_subsystem_info; MAX_CORE_SUBSYSTEM_INFO],
) {
    for raw_sys in raw_subsystem {
        if !raw_sys.ident.is_null() {
            let subsystem = SubSystemInfo::default();

            *subsystem.id.lock().unwrap() = raw_sys.id;
            *subsystem.desc.lock().unwrap() = get_str_from_ptr(raw_sys.desc);
            *subsystem.ident.lock().unwrap() = get_str_from_ptr(raw_sys.ident);

            let roms = unsafe {
                *(raw_sys.roms as *mut [retro_subsystem_rom_info; MAX_CORE_SUBSYSTEM_ROM_INFO])
            };

            for index in 0..raw_sys.num_roms {
                let rom = roms[index as usize];

                let memory = unsafe { *(rom.memory as *mut retro_subsystem_memory_info) };

                subsystem.roms.lock().unwrap().push(SubSystemRomInfo {
                    desc: get_string_mutex_from_ptr(rom.desc),
                    valid_extensions: get_string_mutex_from_ptr(rom.valid_extensions),
                    need_fullpath: Mutex::new(rom.need_fullpath),
                    block_extract: Mutex::new(rom.block_extract),
                    required: Mutex::new(rom.required),
                    num_memory: Mutex::new(rom.num_memory),
                    memory: MemoryInfo {
                        extension: get_string_mutex_from_ptr(memory.extension),
                        type_: Mutex::new(memory.type_),
                    },
                });
            }

            ctx.core.system.subsystem.lock().unwrap().push(subsystem);
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod test_system {
    use crate::{system, test_tools};

    #[test]
    fn test_get_sys_info() {
        let raw_result = test_tools::core::get_raw();

        match raw_result {
            Ok(raw) => {
                let sys_info = system::get_sys_info(&raw);

                assert_eq!(
                    *sys_info.library_name.lock().unwrap().clone(),
                    "Snes9x".to_owned()
                );

                assert_eq!(
                    *sys_info.library_version.lock().unwrap().clone(),
                    "1.62.3 46f8a6b".to_owned()
                );

                assert_eq!(
                    *sys_info.valid_extensions.lock().unwrap().clone(),
                    "smc|sfc|swc|fig|bs|st".to_owned()
                );

                assert_eq!(*sys_info.block_extract.lock().unwrap(), false);

                assert_eq!(*sys_info.need_fullpath.lock().unwrap(), false);
            }
            _ => {}
        }
    }
}
