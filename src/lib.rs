use tch::Tensor;
use rayon::prelude::*;

//Stress testing CPU
pub fn cpu_stress_test()->anyhow::Result<()>{
    let slice = vec![1;1_000_000];
    for i in 1..1_000_000{
        let tensor= Tensor::of_slice(&slice).to_device(tch::Device::Cpu);
        println!("{} {:?}",i,tensor.size());
    }
    ok(());
}

//Stress testing GPU
pub fn gpu_stress_test()->anyhow::Result<()>{
    let slice = vec![1;1_000_000];
    for i in 1..1_000_000{
        let tensor= Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}",i,tensor.size());
    }
    ok(());
}

//Threaded Stress Testing Cpu
pub fn cpu_stress_test_threaded()->anyhow::Result<()>{
    let slice=vec![1;1_000_000];
    (1..1_000_000).into_par_iter().for_each(|i|{
        let tensor = Tensor::of_slice(&slice).to_device(tch::Device::Cpu);
        println!("{} {:?}",i,tensor.size())
    });
    ok(());
}

// Threaded Stress Testing Gpu
pub fn gpu_stress_test_threaded()->anyhow::Result<()>{
    let slice=vec![1;1_000_000];
    (1..1_000_000).into_par_iter().for_each(|i| {
        let tensor=Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}",i,tensor.size());
    });
    ok(());
}