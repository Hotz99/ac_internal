use crate::process;

fn from_proc_dir(
    dir: &std::fs::DirEntry,
    is_internal: bool,
) -> Result<process::Process, process::ProcessErrors> {
    let exe = process::read_exe(dir);

    if !exe.0 {
        return Err(process::ProcessErrors::ProcInvalid);
    }

    let pid = process::path_basename(dir).parse();
    if let Err(_) = pid {
        return Err(process::ProcessErrors::ProcInvalid);
    }

    let process = process::Process {
        pid: pid.unwrap(),
        proc_dir: dir.path().into_os_string().into_string().unwrap(),
        exe: exe.1,
        is_internal,
    };

    Ok(process)
}

pub fn from_current() -> Result<process::Process, process::ProcessErrors> {
    let curr_pid = std::process::id();
    let proc_dir = std::format!("/proc/{}", curr_pid);
    let proc_root = std::path::Path::new("/proc");
    for entry in std::fs::read_dir(proc_root).expect("failed to read /proc dir") {
        let entry = entry.expect("failed to get next entry in /proc dir");
        if entry.path().into_os_string().into_string().unwrap() == proc_dir {
            return from_proc_dir(&entry, true);
        }
    }

    Err(process::ProcessErrors::NotFound)
}
