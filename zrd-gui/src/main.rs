mod actions;
mod editor;
mod text_buffer;
mod theme;

use actions::*;
use editor::{should_exit_with_error, TextEditor};
use gpui::*;
use std::path::PathBuf;
use zrd_core::EditorEngine;

fn resolve_file_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // User provided a file path
        let path = PathBuf::from(&args[1]);
        if path.is_absolute() {
            path
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(path)
        }
    } else {
        // Use default global file
        EditorEngine::default_file_path()
    }
}

fn main() {
    let file_path = resolve_file_path();

    Application::new().run(move |app| {
        // Global quit handler - force exit immediately
        app.on_action(|_: &Quit, _app| {
            let exit_code = if should_exit_with_error() { 1 } else { 0 };
            std::process::exit(exit_code);
        });

        app.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("escape", Quit, None),
            KeyBinding::new("cmd-=", IncreaseFontSize, None),
            KeyBinding::new("cmd--", DecreaseFontSize, None),
            KeyBinding::new("cmd-0", ResetFontSize, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("cmd-shift-k", DeleteLine, None),
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("shift-tab", Outdent, None),
            KeyBinding::new("enter", Newline, None),
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("cmd-backspace", DeleteToBeginningOfLine, None),
            KeyBinding::new("cmd-delete", DeleteToEndOfLine, None),
            KeyBinding::new("cmd-left", MoveToBeginningOfLine, None),
            KeyBinding::new("cmd-right", MoveToEndOfLine, None),
            KeyBinding::new("left", MoveLeft, None),
            KeyBinding::new("right", MoveRight, None),
            KeyBinding::new("up", MoveUp, None),
            KeyBinding::new("down", MoveDown, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("shift-up", SelectUp, None),
            KeyBinding::new("shift-down", SelectDown, None),
            KeyBinding::new("alt-left", MoveWordLeft, None),
            KeyBinding::new("alt-right", MoveWordRight, None),
            KeyBinding::new("alt-shift-left", SelectWordLeft, None),
            KeyBinding::new("alt-shift-right", SelectWordRight, None),
            KeyBinding::new("alt-up", MoveLineUp, None),
            KeyBinding::new("alt-down", MoveLineDown, None),
        ]);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point {
                    x: px(100.0),
                    y: px(100.0),
                },
                size: Size {
                    width: px(800.0),
                    height: px(600.0),
                },
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("zrd".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(8.0), px(8.0))),
            }),
            window_background: WindowBackgroundAppearance::Blurred,
            kind: WindowKind::PopUp,
            ..Default::default()
        };

        let _editor_handle = app.open_window(window_options, |window, app| {
            let path = file_path.clone();
            let editor = app.new(|cx| TextEditor::new(path, cx));
            // Focus the editor so user can start typing immediately
            window.focus(&editor.focus_handle(app));
            editor
        })
        .unwrap();

        // Force exit when window is closed (red X button)
        let _ = app.on_window_closed(move |_app| {
            let exit_code = if should_exit_with_error() { 1 } else { 0 };
            std::process::exit(exit_code);
        });

        app.activate(true);
    });

    // Exit with error code 1 if file wasn't modified (for git integration)
    if should_exit_with_error() {
        std::process::exit(1);
    }
}
