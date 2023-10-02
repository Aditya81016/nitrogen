use rdev::{listen, simulate, Event, EventType, Key};

fn main() {
    let mut keypress: Vec<Key> = vec![];

    // the callback which is passed in listen methodd
    let callback = move |event: Event| {
        // Command::new("clear").status();
        println!("\x1B[2J\x1B[1;1H");
        println!("{:?}", event);
        match event.event_type {
            EventType::KeyPress(key) => match key {
                Key::KeyA => {
                    if !keypress.contains(&Key::Space) {
                        send(&EventType::KeyPress(Key::Space))
                    }
                }
                _ => {
                    if !keypress.contains(&key) {
                        keypress.push(key);
                    }
                }
            },
            EventType::KeyRelease(key) => match key {
                Key::KeyA => {
                    if keypress.contains(&Key::Space) {
                        send(&EventType::KeyRelease(Key::Space))
                    }
                }
                _ => {
                    if keypress.contains(&key) {
                        let index = index_of_key(&keypress, key);
                        if index >= 0 {
                            keypress.remove(index as usize);
                        }
                    }
                }
            },
            _ => {}
        }
    };

    // listens for input events and if any error occured then print the error
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

// this method returns the index of given value in given vector
fn index_of_key(vector: &Vec<Key>, value: Key) -> i32 {
    if let Some(index) = vector.iter().position(|item| item == &value) {
        return index as i32;
    }
    -1
}

fn send(event_type: &EventType) {
    // let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(simulate_error) => {
            println!("We could not send {:0?}\n{:1?}", event_type, simulate_error);
        }
    }
}

// 
