use std::task;
use std::comm::{stream, Port, Chan};

fn main() {
    let (port, chan)    : (Port<int>, Chan<int>) = stream();
    let (dbport, dbchan): (Port<int>, Chan<int>) = stream();

    // Supervisor task, like Resque master.
    do task::spawn_supervised {
        loop {
            if(dbport.recv() == 1) {
                chan.send(1);
            } else {
                chan.send(-1);
                break;
            }
        }
    }

    // Shove some stuff down the 'database' pipe
    dbchan.send(1);
    dbchan.send(1);
    dbchan.send(1);
    dbchan.send(1);
    dbchan.send(-1);

    // Listen to our supervisor task and print results
    loop { 
        let result = port.recv();
        if(result == 1) {
            println(fmt!("%?", result));
        } else {
            break;
        }
    }
}
