use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use ioctls::eviocgrab;
use libc::input_event;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::{fs, io};

const EVIOCGRAB: &std::os::raw::c_int = &1;
const INPUT_EVENT_SIZE: usize = std::mem::size_of::<input_event>();

const GREETING: &str = "Starting G600 Linux controller.

It's a good idea to configure G600 with Logitech Gaming Software before running this program:
 - assign left, right, middle mouse button and vertical mouse wheel to their normal functions
 - assign the G-Shift button to \"G-Shift\"
 - assign all other keys (including horizontal mouse wheel) to arbitrary (unique) keyboard keys
";

const DEVICE_DIR: &str = "/dev/input/by-id/";
const DEVICE_PATH_PREFIX: &str = "usb-Logitech_Gaming_Mouse_G600_";
const DEVICE_PATH_SUFFIX: &str = "-if01-event-kbd";

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let commands = HashMap::from([
        (30, Key::Unicode('1')), // G9
        (31, Key::Unicode('2')), // G10
        (32, Key::Unicode('3')), // G11
        (33, Key::Unicode('4')), // G12
        (34, Key::Unicode('5')), // G13
        (35, Key::Unicode('6')), // G14
        (36, Key::Unicode('7')), // G15
        (37, Key::Unicode('8')), // G16
        (38, Key::Unicode('9')), // G17
        (39, Key::Unicode('0')), // G18
        (4, Key::Tab),           // G19
        (5, Key::Escape),        // G20
        (18, Key::Unicode('h')), // G8
        (19, Key::Unicode('m')), // Wheel Right
        (20, Key::Unicode('i')), // Wheel Left
        (6, Key::F1),            // G-Shift G9
        (7, Key::F2),            // G-Shift G10
        (8, Key::F3),            // G-Shift G11
        (9, Key::F4),            // G-Shift G12
        (10, Key::Unicode('v')), // G-Shift G13
        (11, Key::Unicode('`')), // G-Shift G14
        (12, Key::Unicode('z')), // G-Shift G15 => Num7
        (13, Key::F5),           // G-Shift G16
        (14, Key::Unicode('b')), // G-Shift G17
        (15, Key::Unicode('z')), // G-Shift G18 => Num0
        (16, Key::Unicode('o')), // G-Shift G19
        (17, Key::Unicode('x')), // G-Shift G20
        (21, Key::Unicode('h')), // G-Shift G8
        (22, Key::Unicode('m')), // G-Shift Wheel Right
        (23, Key::Unicode('i')), // G-Shift Wheel Left
        (24, Key::Unicode('u')), // G-Shift Wheel Push
    ]);
    // 98 => enigo.key(Key::Unicode('g7'), action).unwrap(),       // G7 (currently 'cycle sensitivity' => no scancode emitted)
    // 99 => enigo.key(Key::Unicode('shift g7'), action).unwrap(), //  G-Shift G7 (currently 'cycle sensitivity' => no scancode emitted)

    println!("{GREETING}");

    let g600_path = find_g600().unwrap();
    let mut g600 = match File::open(&g600_path) {
        Err(why) => panic!("couldn't open {:?}: {}", &g600_path, why),
        Ok(file) => file,
    };

    unsafe { eviocgrab(g600.as_raw_fd(), EVIOCGRAB) };
    println!("G600 controller started successfully\n");
    loop {
        let events = read_event_batch(&mut g600);
        assert!(events.len() == 2 || events.len() == 3);

        if events[0].type_ != 4 || events[0].code != 4 || events[1].type_ != 1 {
            println!("Unexpected events:");
            println!(
                "type: {:?}, code: {:?}, value: {:?}",
                events[0].type_, events[0].code, events[0].value,
            );
            println!(
                "type: {:?}, code: {:?}, value: {:?}",
                events[1].type_, events[1].code, events[1].value,
            );
            continue;
        }

        let action = if events[1].value == 1 { Press } else { Release };
        let scancode = events[0].value & !0x70000;
        match commands.get(&scancode) {
            Some(key) => enigo.key(*key, action).unwrap(),
            None => println!("Unknown scancode: {:?}\n", scancode),
        }
    }
}

fn read_event_batch(file: &mut File) -> Vec<input_event> {
    let mut events: Vec<input_event> = Vec::new();
    loop {
        let event = read_input_event(file).unwrap();
        events.push(event);
        if event.type_ == 0 && event.code == 0 {
            return events;
        };
    }
}

fn read_input_event(file: &mut File) -> io::Result<input_event> {
    let mut buffer = [0; INPUT_EVENT_SIZE];
    file.read_exact(&mut buffer)?;
    let event: input_event = unsafe { std::mem::transmute(buffer) };
    Ok(event)
}

fn find_g600() -> Result<PathBuf, std::io::Error> {
    fs::read_dir(DEVICE_DIR)?
        .filter_map(Result::ok)
        .find_map(|entry| {
            let path = entry.path();
            let filename = path.file_stem()?.to_str()?;
            if filename.starts_with(DEVICE_PATH_PREFIX) && filename.ends_with(DEVICE_PATH_SUFFIX) {
                Some(path)
            } else {
                None
            }
        })
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find the Logitech G600 file path.",
            )
        })
}
