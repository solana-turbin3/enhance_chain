use std::thread;

use actix_web::{web, App, HttpServer};
use async_channel;
use frontend::FrontendMessage;
use rollupdb::{RollupDB, RollupDBMessage};
use solana_sdk::transaction::Transaction;
use tokio::runtime::Builder;
use crossbeam;
mod frontend;
mod rollupdb;
mod sequencer;
mod settle;
pub mod pool;
pub mod scheduler;
pub mod line_up_queue;
pub mod accounts_db;
pub mod per_db;
pub mod users_handler;
pub mod error;
pub mod processor;
pub mod chain_entrypoint;
// #[actix_web::main]
fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    log::info!("starting HTTP server at http://localhost:8080");


    let (sequencer_sender, sequencer_receiver) = crossbeam::channel::unbounded::<Transaction>();
    let (rollupdb_sender, rollupdb_receiver) = crossbeam::channel::unbounded::<RollupDBMessage>();
    
    // let (sequencer_sender, sequencer_receiver) = async_channel::bounded::<Transaction>(100); // Channel for communication between frontend and sequencer
    // let (rollupdb_sender, rollupdb_receiver) = async_channel::unbounded::<RollupDBMessage>(); // Channel for communication between sequencer and accountsdb
    let (frontend_sender, frontend_receiver) = async_channel::unbounded::<FrontendMessage>(); // Channel for communication between data availability layer and frontend
    // std::thread::spawn(sequencer::run(sequencer_receiver, rollupdb_sender.clone()));
    
    // let rt = Builder::new()
    //     .threaded_scheduler()
    //     .enable_all()
    //     .build()
    //     .unwrap();
    let db_sender2 = rollupdb_sender.clone();
    let fe_2 = frontend_sender.clone();
    
    
    let asdserver_thread = thread::spawn(|| {
        let rt = Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap();



        rt.block_on(async {sequencer::run(sequencer_receiver, db_sender2).unwrap()});
        rt.spawn(RollupDB::run(rollupdb_receiver, fe_2));
    });
    // Create sequencer task
    // tokio::spawn(sequencer::run(sequencer_receiver, rollupdb_sender.clone()));
    // tokio::task::spawn_blocking(|| sequencer::run(sequencer_receiver, rollupdb_sender.clone()) ).await.unwrap();
    // tokio::task::block_in_place(|| sequencer::run(sequencer_receiver, rollupdb_sender.clone()) ).await.unwrap();

    // Create rollup db task (accounts + transactions)
    // tokio::spawn(RollupDB::run(rollupdb_receiver, frontend_sender.clone()));

    // let frontend_receiver_mutex = Arc::new(Mutex::new(frontend_receiver));

     // Spawn the Actix Web server in a separate thread
    let server_thread = thread::spawn(|| {
            // Create a separate Tokio runtime for Actix Web
        let rt2 = Builder::new_multi_thread()
            .worker_threads(4)
            .enable_io()
            .build()
            .unwrap();

        // Create frontend server
        rt2.block_on(async {HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(sequencer_sender.clone()))
                .app_data(web::Data::new(rollupdb_sender.clone()))
                .app_data(web::Data::new(frontend_sender.clone()))
                .app_data(web::Data::new(frontend_receiver.clone()))
                .route("/", web::get().to(frontend::test))
                .route(
                    "/get_transaction",
                    web::post().to(frontend::get_transaction),
                )
                .route(
                    "/submit_transaction",
                    web::post().to(frontend::submit_transaction),
                )
            // .service(
            //     web::resource("/submit_transaction")
            //         .route(web::post().to(frontend::submit_transaction)),
            // )
        })
        .worker_max_blocking_threads(2)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap();
        // tokio::time::sleep(std::time::Duration::from_secs(20)).await;
        });
        });
        server_thread.join().unwrap();

    // rt.shutdown_timeout(std::time::Duration::from_secs(20));


    // Ok(())
}
