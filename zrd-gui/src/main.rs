mod actions;
mod editor;
mod text_buffer;
mod theme;

use actions::*;
use editor::TextEditor;
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
        app.bind_keys([
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
            ..Default::default()
        };

        app.open_window(window_options, |_window, app| {
            let path = file_path.clone();
            app.new(|cx| TextEditor::new(path, cx))
        })
        .unwrap();

        // Quit app when any window is closed (makes it work with git, etc.)
        let _ = app.on_window_closed(|app| {
            app.quit();
        });

        app.activate(true);
    });
}
