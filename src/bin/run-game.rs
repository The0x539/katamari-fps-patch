use std::path::Path;

use win32_util::processes::ProcessAccess;

fn main() {
    let exe = std::env::current_exe().unwrap();
    let dir = exe.parent().unwrap();
    let dll = dir.join("katamari_ffi.dll");
    let pdb = dll.with_extension("pdb");

    let kdr = Path::new("C:/Program Files (x86)/Steam/steamapps/common/Katamari Damacy REROLL/");
    let plugins = kdr.join("katamari_Data/Plugins/");

    for info in win32_util::processes::ProcessInfo::iter().unwrap() {
        let Ok(proc) = info.open(ProcessAccess::QUERY_INFORMATION, false) else {
            continue;
        };
        let Ok(path) = proc.full_path() else {
            continue;
        };
        if path.starts_with(&kdr) {
            info.open(ProcessAccess::TERMINATE, false)
                .unwrap()
                .terminate(0)
                .unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
            break;
        }
    }

    for file in [dll, pdb] {
        std::fs::copy(&file, plugins.join(file.file_name().unwrap())).unwrap();
    }

    std::process::Command::new(kdr.join("katamari.exe"))
        .spawn()
        .unwrap();
}
