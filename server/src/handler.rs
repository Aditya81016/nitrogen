use std::{thread, time};

use rdev::{simulate, Button, EventType, Key};

// handle function which will get the msg
pub fn handle(msg: String) {
    // creates a vector of given commands
    let commands: Vec<&str> = msg.split(" | ").collect();

    // run each command
    for command in commands {
        let mut args: Vec<&str> = command.split_ascii_whitespace().collect();
        // if keyboard command is requested
        if args[0] == "keyboard" {
            args.remove(0);
            handle_keyboard(args);
        } else 
        // if mouse command is requested
        if args[0] == "mouse" {
            args.remove(0);
            handle_mouse(args);
        }
    }
}

// handle keyboard command
fn handle_keyboard(args: Vec<&str>) {
    // if hit is requested
    if args[0] == "hit" {
        // create a delay variable
        let ms;
        if args.len() > 2 {
            ms = args[2].to_owned().parse().unwrap();
        } else {
            ms = 250; // default delay of 250ms
        }
        let delay = time::Duration::from_millis(ms);

        // get the requested key
        let key = get_key(args[1]).unwrap();
        
        // press the request key
        send(&EventType::KeyPress(key));

        // wait for delay to get over
        thread::sleep(delay);

        // release the key after delay is over
        send(&EventType::KeyRelease(key));
    } else 
    
    // if press is requested
    if args[0] == "press" {
        // get the requested key
        let key = get_key(args[1]).unwrap();
        // press the requested key
        send(&EventType::KeyPress(key));
    } else
    
    // if release is requested
     if args[0] == "release" {
        // get the requested key
        let key = get_key(args[1]).unwrap();
        // release the requested key
        send(&EventType::KeyRelease(key));
    }
}

// handle mouse command
fn handle_mouse(args: Vec<&str>) {
    // get the requested button
    let button: Button;
    if args[1] == "Right" {
        button = Button::Right;
    } else {
        button = Button::Left; // Left is default button
    }

    // if click is requested
    if args[0] == "click" {
        // create a delay variable
        let ms;
        if args.len() > 2 {
            ms = args[2].to_owned().parse().unwrap();
        } else {
            ms = 250;
        }
        let delay = time::Duration::from_millis(ms);

        // press the requested button
        send(&EventType::ButtonPress(button));

        // wait for delay to get over
        thread::sleep(delay);

        // release the requested button
        send(&EventType::ButtonRelease(button));
    } else
    
    // if press is requested
     if args[0] == "press" {
        // press the requested button
        send(&EventType::ButtonPress(button));
    } else 
    
    // if release is requested
    if args[0] == "release" {
        // release the requested button
        send(&EventType::ButtonRelease(button));
    } else
    
    // if move is requested
     if args[0] == "move" {
        // get requested coordinates
        let x: f64 = args[1].to_owned().parse().unwrap();
        let y: f64 = args[2].to_owned().parse().unwrap();

        // move the mouse to requested coordinates
        send(&EventType::MouseMove { x, y });
    } else
    
    // if scroll is requested
     if args[0] == "scroll" {

        // get the scroll x, y values
        let x: i64 = args[1].to_owned().parse().unwrap();
        let y: i64 = args[2].to_owned().parse().unwrap();

        // trigger scroll event
        send(&EventType::Wheel {
            delta_x: x,
            delta_y: y,
        });
    }
}

// i found this function from rdev documentaions
fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(simulate_error) => {
            println!(
                "We could not send {:?} because of: \n {:?}",
                event_type, simulate_error
            );
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

// i prompted chatgpt to write this
// this matched the provided key_str and returns respective key
fn get_key(key_str: &str) -> Option<Key> {
    match key_str {
        "Alt" => Some(Key::Alt),
        "AltGr" => Some(Key::AltGr),
        "Backspace" => Some(Key::Backspace),
        "CapsLock" => Some(Key::CapsLock),
        "ControlLeft" => Some(Key::ControlLeft),
        "ControlRight" => Some(Key::ControlRight),
        "Delete" => Some(Key::Delete),
        "DownArrow" => Some(Key::DownArrow),
        "End" => Some(Key::End),
        "Escape" => Some(Key::Escape),
        "F1" => Some(Key::F1),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "Home" => Some(Key::Home),
        "LeftArrow" => Some(Key::LeftArrow),
        "MetaLeft" => Some(Key::MetaLeft),
        "MetaRight" => Some(Key::MetaRight),
        "PageDown" => Some(Key::PageDown),
        "PageUp" => Some(Key::PageUp),
        "Return" => Some(Key::Return),
        "RightArrow" => Some(Key::RightArrow),
        "ShiftLeft" => Some(Key::ShiftLeft),
        "ShiftRight" => Some(Key::ShiftRight),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),
        "UpArrow" => Some(Key::UpArrow),
        "PrintScreen" => Some(Key::PrintScreen),
        "ScrollLock" => Some(Key::ScrollLock),
        "Pause" => Some(Key::Pause),
        "NumLock" => Some(Key::NumLock),
        "BackQuote" => Some(Key::BackQuote),
        "Num1" => Some(Key::Num1),
        "Num2" => Some(Key::Num2),
        "Num3" => Some(Key::Num3),
        "Num4" => Some(Key::Num4),
        "Num5" => Some(Key::Num5),
        "Num6" => Some(Key::Num6),
        "Num7" => Some(Key::Num7),
        "Num8" => Some(Key::Num8),
        "Num9" => Some(Key::Num9),
        "Num0" => Some(Key::Num0),
        "Minus" => Some(Key::Minus),
        "Equal" => Some(Key::Equal),
        "KeyQ" => Some(Key::KeyQ),
        "KeyW" => Some(Key::KeyW),
        "KeyE" => Some(Key::KeyE),
        "KeyR" => Some(Key::KeyR),
        "KeyT" => Some(Key::KeyT),
        "KeyY" => Some(Key::KeyY),
        "KeyU" => Some(Key::KeyU),
        "KeyI" => Some(Key::KeyI),
        "KeyO" => Some(Key::KeyO),
        "KeyP" => Some(Key::KeyP),
        "LeftBracket" => Some(Key::LeftBracket),
        "RightBracket" => Some(Key::RightBracket),
        "KeyA" => Some(Key::KeyA),
        "KeyS" => Some(Key::KeyS),
        "KeyD" => Some(Key::KeyD),
        "KeyF" => Some(Key::KeyF),
        "KeyG" => Some(Key::KeyG),
        "KeyH" => Some(Key::KeyH),
        "KeyJ" => Some(Key::KeyJ),
        "KeyK" => Some(Key::KeyK),
        "KeyL" => Some(Key::KeyL),
        "SemiColon" => Some(Key::SemiColon),
        "Quote" => Some(Key::Quote),
        "BackSlash" => Some(Key::BackSlash),
        "IntlBackslash" => Some(Key::IntlBackslash),
        "KeyZ" => Some(Key::KeyZ),
        "KeyX" => Some(Key::KeyX),
        "KeyC" => Some(Key::KeyC),
        "KeyV" => Some(Key::KeyV),
        "KeyB" => Some(Key::KeyB),
        "KeyN" => Some(Key::KeyN),
        "KeyM" => Some(Key::KeyM),
        "Comma" => Some(Key::Comma),
        "Dot" => Some(Key::Dot),
        "Slash" => Some(Key::Slash),
        "Insert" => Some(Key::Insert),
        "KpReturn" => Some(Key::KpReturn),
        "KpMinus" => Some(Key::KpMinus),
        "KpPlus" => Some(Key::KpPlus),
        "KpMultiply" => Some(Key::KpMultiply),
        "KpDivide" => Some(Key::KpDivide),
        "Kp0" => Some(Key::Kp0),
        "Kp1" => Some(Key::Kp1),
        "Kp2" => Some(Key::Kp2),
        "Kp3" => Some(Key::Kp3),
        "Kp4" => Some(Key::Kp4),
        "Kp5" => Some(Key::Kp5),
        "Kp6" => Some(Key::Kp6),
        "Kp7" => Some(Key::Kp7),
        "Kp8" => Some(Key::Kp8),
        "Kp9" => Some(Key::Kp9),
        "KpDelete" => Some(Key::KpDelete),
        "Function" => Some(Key::Function),
        _ => Some(Key::Unknown(0)), // Handle unknown keys
    }
}
