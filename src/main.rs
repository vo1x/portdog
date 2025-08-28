use clap::{Parser, Subcommand, ValueEnum};

use netstat2::{
    AddressFamilyFlags as AF, ProtocolFlags as PF, ProtocolSocketInfo, get_sockets_info,
};

use sysinfo::{Pid, System};

use std::error::Error;

use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "portdog", version, about = "Ports & processes helper")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Who {
        port: u16,
        #[arg(long, value_enum, default_value_t = Proto::Any)]
        proto: Proto,
    },
    Kill {
        port: u16,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum Proto {
    Tcp,
    Udp,
    Any,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Who { port, proto } => who(port, proto)?,
        Command::Kill { port } => kill_port(port)?,
    }
    Ok(())
}

fn who(port: u16, proto: Proto) -> Result<(), Box<dyn Error>> {
    let af = AF::IPV4 | AF::IPV6;
    let pf = match proto {
        Proto::Tcp => PF::TCP,
        Proto::Udp => PF::UDP,
        Proto::Any => PF::TCP | PF::UDP,
    };
    let sockets = get_sockets_info(af, pf)?;

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut found = false;
    for si in sockets {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(t) if t.local_port == port => {
                found = true;
                print_line(
                    "TCP",
                    t.local_port,
                    Some(t.state.to_string()),
                    &si.associated_pids,
                    &sys,
                );
            }
            ProtocolSocketInfo::Udp(u) if u.local_port == port => {
                found = true;
                print_line("UDP", u.local_port, None, &si.associated_pids, &sys);
            }
            _ => {}
        }
    }

    if !found {
        println!("No process is using port: {port}.");
    }
    Ok(())
}

fn print_line(proto: &str, port: u16, state: Option<String>, pids: &[u32], sys: &System) {
    if pids.is_empty() {
        println!("{proto:>3} :{port:<5}  PID: <unknown>  (insufficient privileges?)");
        return;
    }

    for pid_u32 in pids {
        let pid = Pid::from_u32(*pid_u32);
        if let Some(proc_) = sys.process(pid) {
            let name = proc_.name().to_string_lossy();
            let exe = match proc_.exe() {
                Some(path) => path.display().to_string(),
                None => "<unknown>".to_string(),
            };
            match &state {
                Some(s) => println!(
                    "{proto:>3} :{port:<5}  PID: {pid_u32:<7}  STATE: {s:<12}  NAME: {name}  EXE: {exe}"
                ),
                None => {
                    println!("{proto:>3} :{port:<5}  PID: {pid_u32:<7}  NAME: {name}  EXE: {exe}")
                }
            }
        } else {
            println!("{proto:>3} :{port:<5}  PID: {pid_u32:<7}  (process info unavailable)");
        }
    }
}

fn kill_port(port: u16) -> Result<(), Box<dyn Error>> {
    let pids = collect_pids_for_port(port)?;
    if pids.is_empty() {
        println!("No process is using port: {port}");
        return Ok(());
    }

    println!("Found PIDs on port {port}: {:?}", pids);

    let mut failures = Vec::new();
    for pid in pids {
        if let Err(e) = kill_pid(pid) {
            failures.push((pid, e.to_string()));
        } else {
            println!("Stopped PID {pid}");
        }
    }

    if !failures.is_empty() {
        eprintln!("Some processes could not be killed: ");
        for (pid, error) in failures {
            eprintln!(" PID {pid}: {error}");
        }
    }
    Ok(())
}

fn collect_pids_for_port(port: u16) -> Result<Vec<u32>, Box<dyn Error>> {
    let af = AF::IPV4 | AF::IPV6;
    let pf = PF::TCP | PF::UDP;

    let mut set = HashSet::new();
    for si in get_sockets_info(af, pf)? {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(t) if t.local_port == port => {
                set.extend(si.associated_pids.iter().copied());
            }
            ProtocolSocketInfo::Udp(u) if u.local_port == port => {
                set.extend(si.associated_pids.iter().copied());
            }
            _ => {}
        }
    }

    Ok(set.into_iter().collect())
}

#[cfg(unix)]
fn kill_pid(pid: u32) -> Result<(), Box<dyn Error>> {
    use nix::sys::signal::{Signal::SIGTERM, kill};
    use nix::unistd::Pid as UnixPid;
    kill(UnixPid::from_raw(pid as i32), SIGTERM)?;
    Ok(())
}

#[cfg(windows)]
fn kill_pid(pid: u32) -> Result<(), Box<dyn Error>> {
    use std::process::Command as ProcCommand;
    let status = ProcCommand::new("taskKill")
        .args(["/PID", &pid.to_string()])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err("taskkill failed".into());
    }
}
