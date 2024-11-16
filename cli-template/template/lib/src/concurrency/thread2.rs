use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// attempting to make something threaded myself
pub(crate) fn thread2() {
    const THREADS: usize = 10;
    const WORK_PER_THREAD: usize = 1000;
    const TOTAL_WORK: usize = THREADS * WORK_PER_THREAD;
    let work = Arc::new(Mutex::new(vec![]));
    let results = Arc::new(Mutex::new(vec![]));
    let seq = Arc::new(Mutex::new(0));
    let state = Arc::new(Mutex::new(State::Init));
    let mut handles = vec![];

    // a little thread that just prints the status/progress every second
    {
        let arc_id = Arc::clone(&seq);
        let arc_state = Arc::clone(&state);
        let arc_wrk = Arc::clone(&work);
        let arc_res = Arc::clone(&results);
        handles.push(thread::spawn(move || loop {
            {
                let id = arc_id.lock().unwrap();
                let wrk = arc_wrk.lock().unwrap();
                let res = arc_res.lock().unwrap();

                println!("State: {:?}", arc_state.lock().unwrap());
                println!("Seq: {:?}", id);
                println!("Work: {:?}", wrk.len());
                println!("Result: {:?}", res.len());
                println!();
                if res.len() >= TOTAL_WORK && wrk.is_empty() {
                    println!("Exiting print thread");
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }));
    }
    // create work, if it was only this easy irl
    {
        let mut state = state.lock().unwrap();
        *state = State::SpawingJobCreators;
    }
    for _ in 0..THREADS {
        let arc_id = Arc::clone(&seq);
        let arc_wrk = Arc::clone(&work);
        handles.push(thread::spawn(move || {
            for _ in 0..WORK_PER_THREAD {
                thread::sleep(Duration::from_millis(1000));
                let mut id = arc_id.lock().unwrap();
                *id += 1;

                arc_wrk.lock().unwrap().push(Work {
                    id: id.clone(),
                    msg: "hello".to_string(),
                })
            }
        }));
    }

    // create workers
    {
        let mut state = state.lock().unwrap();
        *state = State::SpawningWorkers;
    }
    for _ in 0..THREADS {
        let arc_wrk = Arc::clone(&work);
        let arc_res = Arc::clone(&results);
        handles.push(thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1000));
            let maybe_work = arc_wrk.lock().unwrap().pop();

            let maybe_result = maybe_work.map(|work| do_work(work));

            if let Some(result) = maybe_result {
                arc_res.lock().unwrap().push(result);
            }
            {
                let res = arc_res.lock().unwrap();
                if res.len() >= TOTAL_WORK {
                    break;
                }
            }
        }));
    }

    {
        let mut state = state.lock().unwrap();
        *state = State::WaitingForWorkers;
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Seq: {:?}", seq.lock().unwrap());
    println!("Work: {:?}", work.lock().unwrap().len());
    println!("Result: {:?}", results.lock().unwrap().len());
}

// dummy function to to simulate doing work
fn do_work(work: Work) -> Result {
    //    thread::sleep(Duration::from_millis(10));
    Result {
        id: work.id,
        msg: work.msg.to_ascii_uppercase(),
    }
}

#[derive(Debug)]
struct Work {
    id: i32,
    msg: String,
}

#[derive(Debug)]
struct Result {
    id: i32,
    msg: String,
}

#[derive(Debug)]
enum State {
    Init,
    SpawingJobCreators,
    SpawningWorkers,
    WaitingForWorkers,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_with_mutex() {
        let wrapped = Arc::new(Mutex::new(String::from("hello")));
        println!("Before: value is: {:?}", wrapped);
        {
            let mut unlocked = wrapped.lock().unwrap();
            *unlocked = (*unlocked.to_uppercase()).parse().unwrap();
            println!("After mut: value is: {:?}", wrapped);
            println!("actual var is: {:?}", unlocked);
        }
        {
            println!("New Scope value is: {:?}", wrapped);
        }
    }
}
