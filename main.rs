extern crate rand;
extern crate num;
extern crate time;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use rand::Rng;
use num::abs;
use num::Float;
use num::integer;
use time::PreciseTime;
use time::Duration;

const trials: i32 = 16384;
const NTHREADS: i32 = 1;
const radius: i32 = 10;
const tpt: i32 = trials / NTHREADS;

//currently gives an answer that is too high.
    //should be ~99 for a radius of 10, not ~120
fn main() {

    // Performence timer
    let start = PreciseTime::now();

    // Creates the two ends of the channel
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();


    for id in 0..NTHREADS {
        // Clones the sender half of the channel for use in this thread
        // Since the clone operation will happen again before the next thread is spawned,
            // each thread gets a unique copy of the sender
        let thread_tx = tx.clone();

        // Spawns a thread
        thread::spawn(move || {

            //Sets up everything we need for the game
            let mut steps = 0;
            let mut result: i32 = 0;
            let mut x = 0;
            let mut rng = rand::thread_rng();

            // Runs the trils
            for k in 0..tpt {

                x = 0;

                // One trial
                loop{

                    let flip = rng.gen();

                    steps = steps + 1;

                    if flip {
                        x = x + 1;
                    } else {
                        x = x - 1;
                    }

                    if abs(x) >= radius {
                        break;
                    }

                }
            }
            // Averages all runs for this thread
            result = steps / tpt;
            //println!("{:?}", result );

            // Sends the results over the channel
            thread_tx.send(result).unwrap();
        });
    }

    // Collect the results
    //This is magic
    let mut steps: Vec<i32> = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        steps.push(rx.recv().unwrap());
    }
    // Sums the results
    let sum = steps.iter().fold(0, |sum, x| sum + x);

    let average = sum as f64 / NTHREADS as f64;

    println!("Average = {} steps", average);

    //perfornence data
    let end = PreciseTime::now();
    let time = start.to(end);

    let mili_time: f64 = time.num_milliseconds() as f64;
    let seconds: f64 = mili_time / 1000.0;

    println!("Runtime = {} seconds", seconds);

}
