use cli_table::{format::Justify, Cell, Style, Table};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    loop {
        // I'll work on a way to make it work for other OS's and to simplify this.
        if std::env::consts::OS == "linux" {
            std::process::Command::new("clear").status().unwrap();
        } else if std::env::consts::OS == "windows" {
            std::process::Command::new("cls").status().unwrap();
        }

        let mut table = vec![];

        for (pid, process) in sys.processes() {
            let cpu_usage = format!("{:.2}%", process.cpu_usage());
            let memory_usage = format!("{:.2}", process.memory() as f32 / 125000.0f32);
            table.push(vec![
                pid.cell(),
                process.name().cell().justify(Justify::Right),
                cpu_usage.cell().justify(Justify::Right),
                memory_usage.cell().justify(Justify::Right),
            ])
        }

        let table_display = table
            .table()
            .title(vec![
                "PID".cell().bold(true),
                "name".cell().bold(true).justify(Justify::Right),
                "cpu".cell().bold(true).justify(Justify::Right),
                "memory".cell().bold(true).justify(Justify::Right),
            ])
            .bold(true)
            .display()
            .unwrap();

        println!("{}", table_display);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
