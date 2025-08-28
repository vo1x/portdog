use clap::{Parser, Subcommand, ValueEnum};

use netstat2::{
    AddressFamilyFlags as AF, ProtocolFlags as PF, ProtocolSocketInfo, get_sockets_info,
};

use sysinfo::{Pid, System};

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
    }
    Ok(())
}

fn who(port: u16, proto: Proto) -> Result<(), Box<dyn std::error::Error>> {
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
