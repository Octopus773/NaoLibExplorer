#![feature(const_option)]
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use tokio;
mod api;
mod naolibexplorer;

use api::*;

const APP_ID: &str = "org.Octopus773.Naolibexplorer";

#[tokio::main]
async fn main() -> glib::ExitCode {
	// Create a new application
	let app = Application::builder().application_id(APP_ID).build();

	// Connect to "activate" signal of `app`
	app.connect_activate(build_ui);

	let res = get_near_stops(47.2059591, -1.5605108).await.unwrap();
	println!("{:#?}", &res);

	// Run the application
	app.run()
}

fn build_ui(app: &Application) {
	// Create a button with label and margins
	let button = Button::builder()
		.label("Press me!")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	// Connect to "clicked" signal of `button`
	button.connect_clicked(|button| {
		// Set the label to "Hello World!" after the button has been clicked on
		button.set_label("Hello World!");
	});

	// Create a window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("My GTK App")
		.child(&button)
		.build();

	// Present window
	window.present();
}
