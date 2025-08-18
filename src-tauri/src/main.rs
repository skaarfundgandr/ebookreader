// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ebookreader_lib::commands::sample;

fn main() {
    sample::run();
}
