use std::process::Command;

pub fn ping_sweep(ip4_addr: &str) {
        let output = Command::new("ping")
            .arg("-c")
            .arg("5")
            .arg(ip4_addr)
            //.spawn()
            .output()
            .expect("ping unavailable");
        //println!("status: {:?}", output.stdout);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}

pub fn run() {
    for num in 1..256 {
        ping_sweep(&format!("192.168.1.{}", num));
    }
}

