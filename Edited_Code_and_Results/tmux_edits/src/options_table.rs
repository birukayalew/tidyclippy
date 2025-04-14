use ::libc;
pub type __u_int = libc::c_uint;
pub type u_int = __u_int;
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: options_table_scope,
    pub minimum: u_int,
    pub maximum: u_int,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub separator: *const libc::c_char,
    pub style: *const libc::c_char,
}
static mut options_table_mode_keys_list: [*const libc::c_char; 3] = [
    b"emacs\0" as *const u8 as *const libc::c_char,
    b"vi\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_clock_mode_style_list: [*const libc::c_char; 3] = [
    b"12\0" as *const u8 as *const libc::c_char,
    b"24\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_keys_list: [*const libc::c_char; 3] = [
    b"emacs\0" as *const u8 as *const libc::c_char,
    b"vi\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_justify_list: [*const libc::c_char; 4] = [
    b"left\0" as *const u8 as *const libc::c_char,
    b"centre\0" as *const u8 as *const libc::c_char,
    b"right\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_position_list: [*const libc::c_char; 3] = [
    b"top\0" as *const u8 as *const libc::c_char,
    b"bottom\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_bell_action_list: [*const libc::c_char; 5] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"any\0" as *const u8 as *const libc::c_char,
    b"current\0" as *const u8 as *const libc::c_char,
    b"other\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_visual_bell_list: [*const libc::c_char; 4] = [
    b"off\0" as *const u8 as *const libc::c_char,
    b"on\0" as *const u8 as *const libc::c_char,
    b"both\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_pane_status_list: [*const libc::c_char; 4] = [
    b"off\0" as *const u8 as *const libc::c_char,
    b"top\0" as *const u8 as *const libc::c_char,
    b"bottom\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_set_clipboard_list: [*const libc::c_char; 4] = [
    b"off\0" as *const u8 as *const libc::c_char,
    b"external\0" as *const u8 as *const libc::c_char,
    b"on\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut options_table: [options_table_entry; 130] = unsafe {
    [
        {
            let mut init = options_table_entry {
                name: b"buffer-limit\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 1 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 50 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"command-alias\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ARRAY,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"split-pane=split-window,splitp=split-window,server-info=show-messages -JT,info=show-messages -JT,choose-window=choose-tree -w,choose-session=choose-tree -s\0"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: b",\0" as *const u8 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-terminal\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"screen\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"escape-time\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 500 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"exit-empty\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"exit-unattached\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"focus-events\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"history-file\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-limit\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 100 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"set-clipboard\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: options_table_set_clipboard_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"terminal-overrides\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ARRAY,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT\0"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: b",\0" as *const u8 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"user-keys\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ARRAY,
                scope: OPTIONS_TABLE_SERVER,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: b",\0" as *const u8 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"activity-action\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_bell_action_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"assume-paste-time\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"base-index\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"bell-action\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_bell_action_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-command\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-shell\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"/bin/sh\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"destroy-unattached\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"detach-on-destroy\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-panes-active-colour\0" as *const u8
                    as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-panes-colour\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 4 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-panes-time\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 1 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1000 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-time\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 750 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"history-limit\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 2000 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"key-table\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"root\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"lock-after-time\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"lock-command\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"lock -np\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"message-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"message-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-command-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"message-command-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-command-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"message-command-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-command-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"message-command-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-command-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=black,fg=yellow\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"message-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=yellow,fg=black\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mouse\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"prefix\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_KEY,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: '\u{2}' as i32 as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"prefix2\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_KEY,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0xffff00000000 as libc::c_ulonglong as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"renumber-windows\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"repeat-time\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 32767 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 500 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"set-titles\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"set-titles-string\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#S:#I:#W - \"#T\" #{session_alerts}\0" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"silence-action\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_bell_action_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 2 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-interval\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 15 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-justify\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_justify_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-keys\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_keys_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"[#S] \0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-left-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-left-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-left-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-length\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 32767 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 10 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-position\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_position_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b" \"#{=21:pane_title}\" %H:%M %d-%b-%y\0" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-right-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-right-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"status-right-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-length\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0 as libc::c_int as u_int,
                maximum: 32767 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 40 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=green,fg=black\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"update-environment\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ARRAY,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"DISPLAY KRB5CCNAME SSH_ASKPASS SSH_AUTH_SOCK SSH_AGENT_PID SSH_CONNECTION WINDOWID XAUTHORITY\0"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"visual-activity\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_visual_bell_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"visual-bell\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_visual_bell_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"visual-silence\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: options_table_visual_bell_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"word-separators\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_SESSION,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b" -_@\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"aggressive-resize\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"allow-rename\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"alternate-screen\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"automatic-rename\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"automatic-rename-format\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}\0"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"clock-mode-colour\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 4 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"clock-mode-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: options_table_clock_mode_style_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"force-height\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"force-width\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"main-pane-height\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 1 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 24 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"main-pane-width\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 1 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 80 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"mode-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"mode-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"mode-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-keys\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: options_table_mode_keys_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=yellow,fg=black\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"monitor-activity\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"monitor-bell\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"monitor-silence\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"other-pane-height\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"other-pane-width\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-active-border-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"pane-active-border-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-active-border-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 2 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"pane-active-border-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-active-border-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"fg=green\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-base-index\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0 as libc::c_int as u_int,
                maximum: (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"pane-border-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"pane-border-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-format\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"\0"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-status\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: options_table_pane_status_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"remain-on-exit\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"synchronize-panes\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-active-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-activity-attr\0" as *const u8
                    as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0x10 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-activity-style\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-activity-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-activity-style\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-activity-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-activity-style\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-activity-style\0" as *const u8
                    as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"reverse\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-bell-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0x10 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-bell-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-bell-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-bell-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-bell-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-bell-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-bell-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"reverse\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-attr\0" as *const u8
                    as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-current-style\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-current-style\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-current-style\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-format\0" as *const u8
                    as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#I:#W#{?window_flags,#{window_flags}, }\0" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-style\0" as *const u8
                    as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-format\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#I:#W#{?window_flags,#{window_flags}, }\0" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-last-attr\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_ATTRIBUTES,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-last-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-last-bg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-last-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-last-fg\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 8 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: b"window-status-last-style\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-last-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-separator\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b" \0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-style\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STYLE,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\0" as *const u8 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"wrap-search\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"xterm-keys\0" as *const u8 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: OPTIONS_TABLE_WINDOW,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: OPTIONS_TABLE_NONE,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0,
                separator: 0 as *const libc::c_char,
                style: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
