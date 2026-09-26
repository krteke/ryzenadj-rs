use ryzenadj::{RyzenAdj, error::Result};

fn main() -> Result<()> {
    let mut ryzen = RyzenAdj::new()?;
    println!("CPU family: {:?}", ryzen.cpu_family());
    println!(
        "BIOS interface version (raw): {}",
        ryzen.bios_interface_version()
    );

    {
        let mut table = ryzen.power_table()?;
        println!("PM Table version: {:#010X}", table.version());
        println!("PM Table float count: {}", table.values().len());
        println!("STAPM (W): {:?}", table.stapm());
        println!("Fast PPT (W): {:?}", table.fast_ppt());
        println!("Slow PPT (W): {:?}", table.slow_ppt());
        println!("Tctl (degrees Celsius): {:?}", table.tctl_temperature());

        table.refresh()?;
        println!("STAPM after refresh (W): {:?}", table.stapm());
    }

    println!("CPU family after releasing table: {:?}", ryzen.cpu_family());
    Ok(())
}
