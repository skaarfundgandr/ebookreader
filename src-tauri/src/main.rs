// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use stellaron_lib::commands::sample;

#[tokio::main]
async fn main() {
    stellaron_lib::api::start();

    sample::run();
}
