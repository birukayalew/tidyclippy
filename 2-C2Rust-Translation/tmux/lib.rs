#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]


extern crate libc;
pub mod src {
pub mod alerts;
pub mod arguments;
pub mod attributes;
pub mod cfg;
pub mod client;
pub mod cmd;
pub mod cmd_attach_session;
pub mod cmd_bind_key;
pub mod cmd_break_pane;
pub mod cmd_capture_pane;
pub mod cmd_choose_tree;
pub mod cmd_command_prompt;
pub mod cmd_confirm_before;
pub mod cmd_copy_mode;
pub mod cmd_detach_client;
pub mod cmd_display_message;
pub mod cmd_display_panes;
pub mod cmd_find;
pub mod cmd_find_window;
pub mod cmd_if_shell;
pub mod cmd_join_pane;
pub mod cmd_kill_pane;
pub mod cmd_kill_server;
pub mod cmd_kill_session;
pub mod cmd_kill_window;
pub mod cmd_list;
pub mod cmd_list_buffers;
pub mod cmd_list_clients;
pub mod cmd_list_keys;
pub mod cmd_list_panes;
pub mod cmd_list_sessions;
pub mod cmd_list_windows;
pub mod cmd_load_buffer;
pub mod cmd_lock_server;
pub mod cmd_move_window;
pub mod cmd_new_session;
pub mod cmd_new_window;
pub mod cmd_paste_buffer;
pub mod cmd_pipe_pane;
pub mod cmd_queue;
pub mod cmd_refresh_client;
pub mod cmd_rename_session;
pub mod cmd_rename_window;
pub mod cmd_resize_pane;
pub mod cmd_respawn_pane;
pub mod cmd_respawn_window;
pub mod cmd_rotate_window;
pub mod cmd_run_shell;
pub mod cmd_save_buffer;
pub mod cmd_select_layout;
pub mod cmd_select_pane;
pub mod cmd_select_window;
pub mod cmd_send_keys;
pub mod cmd_set_buffer;
pub mod cmd_set_environment;
pub mod cmd_set_hook;
pub mod cmd_set_option;
pub mod cmd_show_environment;
pub mod cmd_show_messages;
pub mod cmd_show_options;
pub mod cmd_source_file;
pub mod cmd_split_window;
pub mod cmd_string;
pub mod cmd_swap_pane;
pub mod cmd_swap_window;
pub mod cmd_switch_client;
pub mod cmd_unbind_key;
pub mod cmd_wait_for;
pub mod colour;
pub mod compat {
pub mod fdforkpty;
pub mod fgetln;
pub mod fparseln;
pub mod freezero;
pub mod getdtablecount;
pub mod getopt;
pub mod getprogname;
pub mod imsg;
pub mod imsg_buffer;
pub mod recallocarray;
pub mod setproctitle;
pub mod strtonum;
pub mod unvis;
pub mod vis;
} // mod compat
pub mod control;
pub mod control_notify;
pub mod environ;
pub mod format;
pub mod grid;
pub mod grid_view;
pub mod hooks;
pub mod input;
pub mod input_keys;
pub mod job;
pub mod key_bindings;
pub mod key_string;
pub mod layout;
pub mod layout_custom;
pub mod layout_set;
pub mod log;
pub mod mode_tree;
pub mod names;
pub mod notify;
pub mod options;
pub mod options_table;
pub mod osdep_linux;
pub mod paste;
pub mod r#proc;
pub mod resize;
pub mod screen;
pub mod screen_redraw;
pub mod screen_write;
pub mod server;
pub mod server_client;
pub mod server_fn;
pub mod session;
pub mod status;
pub mod style;
pub mod tmux;
pub mod tty;
pub mod tty_acs;
pub mod tty_keys;
pub mod tty_term;
pub mod utf8;
pub mod variadic;
pub mod window;
pub mod window_buffer;
pub mod window_client;
pub mod window_clock;
pub mod window_copy;
pub mod window_tree;
pub mod xchecks {
pub mod xchecks;
} // mod xchecks
pub mod xmalloc;
pub mod xterm_keys;
} // mod src
