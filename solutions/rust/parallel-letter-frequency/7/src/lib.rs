use std::{collections::HashMap, sync::{mpsc, Arc}, thread};


pub fn frequency<'a>(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    println!("MAIN START");
    thread::scope(|scope| {
        let chunk_size = input.len() as f32 / worker_count as f32;
        
        let text_letters = Arc::new(input);
        let (tx, rx) = mpsc::channel();
        let _ = (0..worker_count).map(|worker_number| {
            let text_letters = Arc::clone(&text_letters);
            let tx = tx.clone();
            scope.spawn(move || {
                println!("WORKER {worker_number} START");
                let chunk_start: usize = (chunk_size*worker_number as f32).round() as usize;
                let chunk_end = (chunk_size*(worker_number+1) as f32).round() as usize;
            
                tx.send(
                    text_letters.iter()
                        .skip(chunk_start)
                        .take(chunk_end-chunk_start)
                        .map(|sentence| sentence.chars().filter(|c| c.is_alphabetic()))
                        .flatten()
                        .map(|c| c.to_lowercase().next().unwrap())
                        .fold(HashMap::new(), |mut counts, current| {
                            *counts.entry(current).or_insert(0) += 1;
                            counts
                        }
                    )
                ).unwrap();
                println!("WORKER {worker_number} END");
            });
        }).collect::<Vec<_>>();
        (0..worker_count).map(|_| rx.recv().unwrap())
            .fold(
                HashMap::new(), 
                |mut totals, next| {
                    let _ = next.into_iter().for_each(|pair| {
                        *totals.entry(pair.0 as char).or_insert(0) += pair.1;
                    });
                    totals
                }
            )
    })
}
