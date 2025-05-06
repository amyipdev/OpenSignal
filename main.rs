mod lib;
pub(crate) mod code;
pub(crate) mod workbench;

pub(crate) use lib::BUILDER;
pub(crate) use lib::WINDOW;
pub(crate) use lib::URI;

use gtk::prelude::*;
use adw::prelude::*;

use gtk::glib::translate::ToGlibPtr;

gtk_blueprint::gen_blp_map!("");

fn main() -> gtk::glib::ExitCode {
    gtk::gio::resources_register_include!("icons.gresource");
    gtk::init();
    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
    let icon_theme = gtk::IconTheme::for_display(&gtk::gdk::Display::default().unwrap());
    icon_theme.add_resource_path("/net/amyip/OpenSignal/icons/");

    let app = adw::Application::new(Some("net.amyip.OpenSignal"), Default::default());
    app.set_resource_base_path(Some("/net/amyip/OpenSignal"));
    app.connect_activate(|app| {
        let builder = gtk::Builder::from_string(gtk_blueprint::get_blp!("./main.blp"));
        let content: gtk::Stack = builder.object("mainstack").unwrap();
        let window = adw::Window::new();
        window.set_application(Some(app));
        window.set_title(Some(&format!("OpenSignal v{}", env!("CARGO_PKG_VERSION"))));
        window.set_default_size(800, 400);
        window.set_content(Some(&content));

        lib::set_builder(builder.to_glib_full());
        lib::set_window(window.to_glib_full());
        lib::set_base_uri(std::ffi::CString::new("/net/amyip/OpenSignal").unwrap().as_ptr());

        code::main();
        window.present();
    });
    app.run()
}
