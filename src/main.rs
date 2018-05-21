extern crate gtk;
use gtk::*;
use std::process;

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Initialize the UI's initial state
    let app = App::new();

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
}

pub struct Content {
    pub container: Box,
    pub pixels: Box,
    pub message: Label,
}

const CSS: &str = include_str!("../css/main.css");
impl App {
    fn new() -> App {
        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a the headerbar and it's associated content.
        let header = Header::new();
        // Create new contnet window
        let content = Content::new();

        // SET CUSTOM CSS STYLING
        let screen = window.get_screen().unwrap();
        let provider = CssProvider::new();
        let _ = CssProviderExt::load_from_data(&provider, CSS.as_bytes());
        StyleContext::add_provider_for_screen(&screen, &provider, STYLE_PROVIDER_PRIORITY_USER);

        // Set size
        //window.set_default_size(600, 350);
        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title("|___CANVAS___|");
        // Set the window manager class.
        window.set_wmclass("app-name", "App name");
        // The icon the app will display.
        Window::set_default_icon_name("iconname");
        // Add content container
        window.add(&content.container);

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return our main application state
        App { window, header, content }
    }
}

impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let container = HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title("App Name");
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Returns the header and all of it's state
        Header { container }
    }
}

impl Content {
    fn new() -> Content {
        let container = Box::new(Orientation::Vertical, 0);
        let pixels = Box::new(Orientation::Horizontal, 0);

        let pixel1 = Button::new();
        let pixel2 = Button::new();

        pixel1.get_style_context().map(|c| c.add_class("pixel_button"));
        pixel2.get_style_context().map(|c| c.add_class("pixel_button"));

        // let health_label = Label::new("Current Health:");
        // let health = Label::new("10");

        pixels.set_halign(Align::Center);
        pixel1.set_halign(Align::Start);
        pixel2.set_halign(Align::Start);

        pixels.pack_start(&pixel1, false, false, 0);
        pixels.pack_start(&pixel2, true, true, 0);

        let message = Label::new("Select color: ");

        let color_button = ColorButton::new();
        println!("{}", color_button.get_rgba());

        // Add everything to our vertical box
        container.pack_start(&pixels, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&message, true, false, 0);
        container.pack_start(&color_button, true, false, 0);

        Content { container, pixels, message }

    }
}
