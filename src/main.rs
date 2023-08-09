use clap::Parser;

#[derive(Parser)]

#[clap(
    version="1.0",
    author="Dewa Sahu",
    about="stress testing gpu and cpu threaded and non threaded"
)]
struct Cli{
    #[clap(subcommand)]
    command:Option<Commands>,
}

#[derive(Parser)]
enum Commands{
    #[clap(version="1.0",author="Dewa Sahu")]
    Cpu{},
    Gpu{},
    TGpu{},
    TCpu{},
}




fn main() {
    let args=Cli::parse();
    match args.command {
        Some(Commands::Cpu {})=>{
            println!("Stress Testing Cpu --normal");
            GpuStress::cpu_stress_test();
        },
        Some(Commands::Gpu {})=>{
            println!("Stress Testing Gpu --normal");
            GpuStress::gpu_stress_test();
        },
        Some(Commands::TGpu {})=>{
            println!("Stress Testing Gpu --threaded");
            GpuStress::gpu_stress_test_threaded();
        },
        Some(Commands::TCpu {})=>{
            println!("Stress Testing Cpu --threaded");
            GpuStress::cpu_stress_test_threaded();
        },
        None=>println!("no command provided")
    }

}
