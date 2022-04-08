use shared;
use server;
use client;

gflags::define! {
    // Run Lany as client (TV + controller)
    --client = false
    // Run Lany as server (PC + game)
    --server = true
    // Port to bind to
    --port = 42069
    // S.O.S
    -h, --help = false
}

fn main() {
    gflags::parse();

    if SERVER.flag && CLIENT.flag {
        println!("Cannot run as Client and Serve at the same time.");
        gflags::print_help_and_exit(1);
    }

    let args = process_flags();

    if SERVER.flag {
        server::run(args);
    }else if CLIENT.flag {
        client::run(args);
    }else {
        println!("Something weird happened");
    }
}

fn process_flags() -> shared::RunnerArgs {
    return shared::RunnerArgs{
        port: PORT.flag
    }
}
