#include <linux/input.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>

struct input_event events[64];
const char kDir[] = "/dev/input/by-id/";
const char kPrefix[] = "usb-Logitech_Gaming_Mouse_G600_";
const char kSuffix[] = "-if01-event-kbd";

// ADD KEY->COMMAND MAPPINGS HERE:
const char *downCommands[] = {
  // REGULAR KEYS
  [30] = "ydotool key l+m+a+o", // G9 types lmao
  [31] = "ydotool key 29:1 17:1 17:0 29:0", // G10 closes tab
  [32] = "ydotool key 29:1 33:1 33:0 29:0", // G11 search
  [33] = "ydotool key 29:1 47:1 47:0 29:0", // G12 paste
  [34] = "ydotool key 125:1 105:1 105:0 125:0", // G13 window left
  [35] = "ydotool key 29:1 20:1 20:0 29:0", // G14 new tab
  [36] = "ydotool key 29:1 46:1 46:0 29:0", // G15 copy
  [37] = "ydotool key 125:1 106:1 106:0 125:0", // G16 window right
  [38] = "ydotool key 125:1 103:1 103:0 125:0", // G17 full window
  [46] = "ydotool key 87:1 87:0", // G18 vscode step in
  [56] = "ydotool key 68:1 68:0", // G19 vscode step over
  [48] = "ydotool key 63:1 63:0", // G20 vscode start debug
  
  [54] = "ydotool key 29:1 49:1 49:0 29:0", // G7 new window
  [49] = "ydotool key 107:1 107:0", // Wheel Right goto end of line
  [47] = "ydotool key 102:1 102:0", // Wheel Left goto start of line

  // G-SHIFT KEYS
  [5] = "ydotool key 125:1 2:1 2:0 125:0", // G-Shift G9 start/open application 1
  [4] = "ydotool key 125:1 3:1 3:0 125:0", // G-Shift G10 start/open application 2
  [11] = "ydotool key 125:1 4:1 4:0 125:0", // G-Shift G11 start/open application 3
  [6] = "ydotool key 29:1 42:1 47:1 47:0 42:0 29:0", // G-Shift G12 paste in terminal
  [10] = "ydotool key 29:1 104:1 104:0 29:0", // G-Shift G13 switch to left tab
  [12] = "ydotool key 29:1 42:1 20:1 20:0 42:0 29:0", // G-Shift G14 new tab in terminal
  [7] = "ydotool key 29:1 42:1 46:1 46:0 42:0 29:0", // G-Shift G15 copy in terminal
  [9] = "ydotool key 29:1 109:1 109:0 29:0", // G-Shift G16 switch to right tab
  [13] = "ydotool key 125:1 108:1 108:0 125:0", // G-Shift G17 un-fullscreen
  [8] = "", // G-Shift G18
  [15] = "", // G-Shift G19
  [14] = "ydotool key 28:1 28:0", // G-Shift G20 Enter
  
  [21] = "ydotool key 29:1 42:1 49:1 49:0 42:0 29:0", // G-Shift G7 new terminal window
  [28] = "", // G-Shift Wheel Right
  [29] = "" // G-Shift Wheel Left

};
// You can add different commands when the button is lifted here, formatted like above
const char *upCommands[64];

int starts_with(const char* haystack, const char* prefix) {
  size_t prefix_length = strlen(prefix), haystack_length = strlen(haystack);
  if (haystack_length < prefix_length) return 0;
  return strncmp(prefix, haystack, prefix_length) == 0;
}

int ends_with(const char* haystack, const char* suffix) {
  size_t suffix_length = strlen(suffix), haystack_length = strlen(haystack);
  if (haystack_length < suffix_length) return 0;
  size_t haystack_end = haystack + haystack_length - suffix_length;
  return strncmp(suffix, haystack_end, suffix_length) == 0;
}

// Returns non-0 on error.
int find_g600(char *path) {
  //*path = kDir;
  DIR *dir;
  struct dirent *ent;
  if (!(dir = opendir(kDir))) {
    return 1;
  }
  while ((ent = readdir(dir))) {
    if (starts_with(ent->d_name, kPrefix) && ends_with(ent->d_name, kSuffix)) {
      strcpy(path, kDir);
      strcat(path, ent->d_name);

      printf("full path is %s\n", path);

      //*path += ent->d_name;
      closedir(dir);
      return 0;
    }
  }
  closedir(dir);
  return 2;
}

int main() {
  printf("Starting G600 Linux controller.\n\n");
  printf("It's a good idea to configure G600 with Logitech Gaming Software before running this program:\n");
  printf(" - assign left, right, middle mouse button and vertical mouse wheel to their normal functions\n");
  printf(" - assign the G-Shift button to \"G-Shift\"\n");
  printf(" - assign all other keys (including horizontal mouse wheel) to arbitrary (unique) keyboard keys\n");
  printf("\n");    
  char path[1024];
  int find_error = find_g600(&path);
  if (find_error) {
    printf("Error: Couldn't find G600 input device.\n");
    switch(find_error) {
    case 1: 
      printf("Suggestion: Maybe the expected directory (%s) is wrong. Check whether this directory exists and fix it by editing \"g600.c\".\n", kDir);
      break;
    case 2:
      printf("Suggestion: Maybe the expected device prefix (%s) is wrong. Check whether a device with this prefix exists in %s and fix it by editing \"g600.cpp\".\n", kPrefix, kDir);
      break;
    }
    printf("Suggestion: Maybe a permission is missing. Try running this program with with sudo.\n");
    return 1;
  }
  int fd = open(path, O_RDONLY);
  if (fd < 0) {
    printf("Error: Couldn't open \"%s\" for reading.\n", path);
    printf("Reason: %s.\n", strerror(errno));
    printf("Suggestion: Maybe a permission is missing. Try running this program with with sudo.\n");
    return 1;
  }

  ioctl(fd, EVIOCGRAB, 1);
  printf("G600 controller started successfully.\n\n");
  while (1) {
    size_t n = read(fd, events, sizeof(events));
    if (n <= 0) return 2;
    if (n < sizeof(struct input_event) * 2) continue;
    if (events[0].type != 4) continue;
    if (events[0].code != 4) continue;
    if (events[1].type != 1) continue;
    int pressed = events[1].value;
    int scancode = events[0].value & ~0x70000;

    const char* actionStr = (pressed) ? "Pressed" : "Released";
    printf("%s scancode %d.\n",actionStr, scancode);

    const char *downCommand = downCommands[scancode], *upCommand = upCommands[scancode];
    const char *cmdToRun = (pressed) ? downCommand : upCommand;
    if (!cmdToRun || !strlen(cmdToRun)) continue;

    printf("Executing: \"%s\"\n", cmdToRun);
    system(cmdToRun);
    printf("\n");
  }
  
  close(fd);
}
