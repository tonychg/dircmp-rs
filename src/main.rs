use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use log::{debug, info};

use clap::Parser;
use jwalk::WalkDir;
use rayon::prelude::*;

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    source: String,
    target: String,

    #[arg(short, long, default_value_t = false)]
    print: bool,

    #[arg(short, long)]
    exclude: Vec<String>,
}

struct Index {
    pub paths: HashSet<String>,
}

impl Index {
    fn new() -> Index {
        Index {
            paths: HashSet::new(),
        }
    }

    fn add(&mut self, path: &str) {
        debug!("New valid path : {}", path);
        self.paths.insert(path.to_string());
    }
}

fn construct_index(path: &str) -> Index {
    let mut index = Index::new();

    info!("Start indexing {}", path);

    for entry in WalkDir::new(path).sort(true) {
        if let Ok(entry) = entry {
            if let Ok(striped_entry) = entry.path().strip_prefix(path) {
                index.add(&striped_entry.display().to_string());
            }
        }
    }

    info!("Finished to index {} : {}", path, index.paths.len());

    return index;
}

fn diff_chunk(chunk: &[String], target: &Index) -> Vec<String> {
    let mut processed_result = Vec::new();

    for path in chunk {
        if !target.paths.contains(path) {
            processed_result.push(path.to_string());
        }
    }

    return processed_result;
}

fn diff(source: &str, target: &str) -> Index {
    let source = construct_index(source);
    let target = construct_index(target);
    let result = Arc::new(Mutex::new(Vec::new()));

    info!("Starting to diff");

    let chunk_size = source.paths.len() / num_cpus::get();

    info!("Chunk size: {}", chunk_size);

    source
        .paths
        .into_iter()
        .collect::<Vec<String>>()
        .par_chunks(chunk_size)
        .for_each(|chunk| {
            let chunk_result = diff_chunk(chunk, &target);
            let mut results_lock = result.lock().unwrap();
            results_lock.extend(chunk_result);
        });

    let final_result = result.lock().unwrap();
    let mut result = Index::new();
    for path in final_result.iter() {
        result.add(path);
    }

    info!("Finished to diff");

    return result;
}

fn print_diff(diff: &Index) {
    for path in diff.paths.iter() {
        println!("{}", path);
    }
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    info!("source: {}", cli.source);
    info!("target: {}", cli.target);

    let result = diff(&cli.source, &cli.target);

    info!(
        "{} of files in {} and not in {}",
        result.paths.len(),
        cli.source,
        cli.target
    );

    if cli.print {
        print_diff(&result);
    }
}

#[cfg(test)]
mod tests {}
