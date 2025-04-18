// #include <linux/input.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>
// #include <dirent.h>
// #include <errno.h>
// #include <fcntl.h>
// #include <unistd.h>

use enigo::{
    // Direction::{Click, Press, Release},
    Direction::{Press, Release},
    Enigo,
    Key,
    Keyboard,
    Settings,
};
use ioctls::eviocgrab;
use libc::input_event;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::{fs, io};

// struct input_event events[64];
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
    // let mut down_commands: HashMap<i32, Box<dyn FnMut() -> ()>> = HashMap::new();
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
                                 // 98 => enigo.key(Key::Unicode('g7'), action).unwrap(),       // G7 (currently 'cycle sensitivity' => no scancode emitted)
                                 // 99 => enigo.key(Key::Unicode('shift g7'), action).unwrap(), //  G-Shift G7 (currently 'cycle sensitivity' => no scancode emitted)
    ]);
    println!("{GREETING}");

    // char path[1024];
    // int find_error = find_g600(&path);
    //   if (find_error) {
    //     printf("Error: Couldn't find G600 input device.\n");
    //     switch(find_error) {
    //     case 1:
    //       printf("Suggestion: Maybe the expected directory (%s) is wrong. Check whether this directory exists and fix it by editing \"g600.c\".\n", K_DIR);
    //       break;
    //     case 2:
    //       printf("Suggestion: Maybe the expected device prefix (%s) is wrong. Check whether a device with this prefix exists in %s and fix it by editing \"g600.cpp\".\n", K_PREFIX, K_DIR);
    //       break;
    //     }
    //     printf("Suggestion: Maybe a permission is missing. Try running this program with with sudo.\n");
    //     return 1;
    //   }
    let g600_path = find_g600().unwrap();
    //   int fd = open(path, O_RDONLY);
    //   if (fd < 0) {
    //     printf("Error: Couldn't open \"%s\" for reading.\n", path);
    //     printf("Reason: %s.\n", strerror(errno));
    //     printf("Suggestion: Maybe a permission is missing. Try running this program with with sudo.\n");
    //     return 1;
    //   }
    let mut g600 = match File::open(&g600_path) {
        Err(why) => panic!("couldn't open {:?}: {}", &g600_path, why),
        Ok(file) => file,
    };

    //   ioctl(fd, EVIOCGRAB, 1);
    unsafe { eviocgrab(g600.as_raw_fd(), EVIOCGRAB) };
    //   printf("G600 controller started successfully.\n\n");
    println!("G600 controller started successfully\n");
    //   while (1) {
    loop {
        //     size_t n = read(fd, events, sizeof(events));
        //     if (n <= 0) return 2;
        //     if (n < sizeof(struct input_event) * 2) continue;
        let events = read_event_batch(&mut g600);
        assert!(events.len() == 2 || events.len() == 3);

        //     if (events[0].type != 4) continue;
        //     if (events[0].code != 4) continue;
        //     if (events[1].type != 1) continue;
        if events[0].type_ != 4 || events[0].code != 4 || events[1].type_ != 1 {
            continue;
        }
        //     int pressed = events[1].value;
        //     int scancode = events[0].value & ~0x70000;
        //     const char* actionStr = (pressed) ? "Pressed" : "Released";
        //     printf("%s scancode %d.\n",actionStr, scancode);
        let action = if events[1].value == 1 { Press } else { Release };
        let scancode = events[0].value & !0x70000;
        // println!("{:?}\n", scancode);

        match commands.get(&scancode) {
            Some(key) => enigo.key(*key, action).unwrap(),
            None => println!("{:?}\n", scancode),
        }

        // println!("{:?} {:?}\n", command, if pressed { "down" } else { "up" });

        //     const char *downCommand = downCommands[scancode], *upCommand = upCommands[scancode];
        //     const char *cmdToRun = (pressed) ? downCommand : upCommand;
        //     if (!cmdToRun || !strlen(cmdToRun)) continue;
        //
        //     printf("Executing: \"%s\"\n", cmdToRun);
        //     system(cmdToRun);
        //     printf("\n");
        //   }
        //
        //   close(fd);
        // }
    }
}
// // ADD KEY->COMMAND MAPPINGS HERE:
// const char *downCommands[] = {
//   // REGULAR KEYS
//   [30] = "ydotool key l+m+a+o", // G9 types lmao
//   [31] = "ydotool key 29:1 17:1 17:0 29:0", // G10 closes tab
//   [32] = "ydotool key 29:1 33:1 33:0 29:0", // G11 search
//   [33] = "ydotool key 29:1 47:1 47:0 29:0", // G12 paste
//   [34] = "ydotool key 125:1 105:1 105:0 125:0", // G13 window left
//   [35] = "ydotool key 29:1 20:1 20:0 29:0", // G14 new tab
//   [36] = "ydotool key 29:1 46:1 46:0 29:0", // G15 copy
//   [37] = "ydotool key 125:1 106:1 106:0 125:0", // G16 window right
//   [38] = "ydotool key 125:1 103:1 103:0 125:0", // G17 full window
//   [46] = "ydotool key 87:1 87:0", // G18 vscode step in
//   [56] = "ydotool key 68:1 68:0", // G19 vscode step over
//   [48] = "ydotool key 63:1 63:0", // G20 vscode start debug
//
//   [54] = "ydotool key 29:1 49:1 49:0 29:0", // G7 new window
//   [49] = "ydotool key 107:1 107:0", // Wheel Right goto end of line
//   [47] = "ydotool key 102:1 102:0", // Wheel Left goto start of line
//
//   // G-SHIFT KEYS
//   [5] = "ydotool key 125:1 2:1 2:0 125:0", // G-Shift G9 start/open application 1
//   [4] = "ydotool key 125:1 3:1 3:0 125:0", // G-Shift G10 start/open application 2
//   [11] = "ydotool key 125:1 4:1 4:0 125:0", // G-Shift G11 start/open application 3
//   [6] = "ydotool key 29:1 42:1 47:1 47:0 42:0 29:0", // G-Shift G12 paste in terminal
//   [10] = "ydotool key 29:1 104:1 104:0 29:0", // G-Shift G13 switch to left tab
//   [12] = "ydotool key 29:1 42:1 20:1 20:0 42:0 29:0", // G-Shift G14 new tab in terminal
//   [7] = "ydotool key 29:1 42:1 46:1 46:0 42:0 29:0", // G-Shift G15 copy in terminal
//   [9] = "ydotool key 29:1 109:1 109:0 29:0", // G-Shift G16 switch to right tab
//   [13] = "ydotool key 125:1 108:1 108:0 125:0", // G-Shift G17 un-fullscreen
//   [8] = "", // G-Shift G18
//   [15] = "", // G-Shift G19
//   [14] = "ydotool key 28:1 28:0", // G-Shift G20 Enter
//
//   [21] = "ydotool key 29:1 42:1 49:1 49:0 42:0 29:0", // G-Shift G7 new terminal window
//   [28] = "", // G-Shift Wheel Right
//   [29] = "" // G-Shift Wheel Left
//
// };

// // You can add different commands when the button is lifted here, formatted like above
// const char *upCommands[64];
//
// int starts_with(const char* haystack, const char* prefix) {
//   size_t prefix_length = strlen(prefix), haystack_length = strlen(haystack);
//   if (haystack_length < prefix_length) return 0;
//   return strncmp(prefix, haystack, prefix_length) == 0;
// }
//
// int ends_with(const char* haystack, const char* suffix) {
//   size_t suffix_length = strlen(suffix), haystack_length = strlen(haystack);
//   if (haystack_length < suffix_length) return 0;
//   size_t haystack_end = haystack + haystack_length - suffix_length;
//   return strncmp(suffix, haystack_end, suffix_length) == 0;
// }
//

fn read_event_batch(file: &mut File) -> Vec<input_event> {
    let mut events: Vec<input_event> = Vec::new();
    // println!();
    loop {
        let event = read_input_event(file).unwrap();
        // println!("({:?}/{:?}): {:?}", event.type_, event.code, event.value);
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

// // Returns non-0 on error.
// int find_g600(char *path) {
fn find_g600() -> Result<PathBuf, std::io::Error> {
    // struct dirent *ent;
    // if (!(dir = opendir(K_DIR))) {
    //     return 1;
    // }
    // while ((ent = readdir(dir))) {
    // for entry in fs::read_dir(K_DIR)? {
    //     let path = entry?.path();
    //     if let Some(stem) = path.file_stem() {
    //         if let Some(filename) = stem.to_str() {
    //             println!("Found a path: {:}", filename);
    //             if filename.starts_with(K_PREFIX) && filename.ends_with(K_SUFFIX) {
    //                 println!("possible G600-Path: {:}", filename);
    //                 // return filename
    //             }
    //         }
    // }
    //     if (starts_with(ent->d_name, K_PREFIX) && ends_with(ent->d_name, K_SUFFIX)) {
    //       strcpy(path, K_DIR);
    //       strcat(path, ent->d_name);
    //
    //       printf("full path is %s\n", path);
    //
    //       //*path += ent->d_name;
    //       closedir(dir);
    //       return 0;
    //     }
    //   }
    //   closedir(dir);
    //   return 2;
    // }
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
