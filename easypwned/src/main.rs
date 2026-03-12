use std::future::IntoFuture;
use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};

use easypwned_bloom::bloom::bloom_get;

#[derive(Parser, Debug)]
pub struct Opt {
    #[arg(long = "bloomfile", default_value = "easypwned.bloom")]
    bloomfile: String,
    #[arg(long = "bind", default_value = "0.0.0.0:3342")]
    bind: String,
}

#[tokio::main]
async fn main() -> ::anyhow::Result<(), ::anyhow::Error> {
    let opt: Opt = Opt::parse();

    println!("reading bloom filter file {}", &opt.bloomfile);
    let bloom = match bloom_get(&opt.bloomfile) {
        Ok(b) => b,
        Err(e) => {
            println!("could not get bloom {}", e);
            panic!();
        }
    };
    println!("finished reading bloom filter file {}", &opt.bloomfile);

    let checks = vec![
        "0000000CAEF405439D57847A8657218C618160B2",
        "0000000CAEF405439D57847A8657218C618160BX",
    ];

    for check in checks {
        println!(
            "check: {} -> {:?}",
            check,
            bloom.check(&check.as_bytes().to_vec())
        );
    }

    let bloom_state = Arc::new(bloom);
    let app = easypwned::create_router(bloom_state);

    let addr = opt.bind.parse::<SocketAddr>().expect("");
    println!("listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    let axum_handle = axum::serve(listener, app);

    let mut sig_quit = signal(SignalKind::quit())?;
    let mut sig_term = signal(SignalKind::terminate())?;

    ::tokio::select! {
        axum = axum_handle.into_future() => {
            axum?;
            panic!("axum quitted")
        },
        _ = sig_quit.recv() => {
            println!("Signal quit, quit.");
        },
        _ = sig_term.recv() => {
            println!("Signal term, quit.");
        }
        _ = ::tokio::signal::ctrl_c() => {
            println!("Signal ctrl_c, quit.");
        }
    }

    Ok(())
}
