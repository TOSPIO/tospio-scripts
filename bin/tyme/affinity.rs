use libc::{
    cpu_set_t,
    pid_t,
    sched_getaffinity,
    sched_setaffinity,
    CPU_SET
};
use std::io::{Error};
use std::mem;
use std::mem::{size_of};

fn cpu_set(cpu: usize, affinity: &mut cpu_set_t) {
    unsafe {
        CPU_SET(cpu, affinity);
    }
}

pub fn get_pid_affinity(pid: pid_t) -> Result<cpu_set_t, Error> {
    unsafe {
        let mut c_affinity: cpu_set_t = mem::uninitialized();
        let cpusetsize = size_of::<cpu_set_t>();
        let ret = sched_getaffinity(pid, cpusetsize, &mut c_affinity);
        if ret == 0 {
            Ok(c_affinity)
        } else {
            Err(Error::last_os_error())
        }
    }
}

pub fn get_affinity() -> Result<cpu_set_t, Error> {
    get_pid_affinity(0)
}

pub fn set_pid_affinity(pid: pid_t, cpu_set: &cpu_set_t) -> Result<(), Error> {
    unsafe {
        let cpusetsize = size_of::<cpu_set_t>();
        let ret = sched_setaffinity(pid, cpusetsize, cpu_set);
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

pub fn set_affinity(cpu_set: &cpu_set_t) -> Result<(), Error> {
    set_pid_affinity(0, cpu_set)
}

pub fn assign_pid_cpu(pid: pid_t, cpu: usize) -> Result<(), Error> {
    let mut affinity = get_pid_affinity(pid)?;
    cpu_set(cpu, &mut affinity);
    Ok(())
}

pub fn assign_cpu(cpu: usize) -> Result<(), Error> {
    assign_pid_cpu(0, cpu)
}
