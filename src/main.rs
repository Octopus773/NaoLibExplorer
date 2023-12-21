#![feature(const_option)]
use std::rc::Rc;
use std::thread;
use std::{cell::Cell, time::Duration};

use gtk::{glib, glib::clone, prelude::*, ApplicationWindow, Button, Paned};
use tokio;
mod api;
mod naolibexplorer;

use api::*;

const APP_ID: &str = "org.Octopus773.Naolibexplorer";

fn main() -> glib::ExitCode {
	// Create a new application
	let app = adw::Application::builder().application_id(APP_ID).build();

	// Connect to "activate" signal of `app`
	app.connect_activate(build_ui);

	// Run the application
	app.run()
}

fn build_ui(app: &adw::Application) {
	// Create a button with label and margins
	let button = Button::builder()
		.label("Increase")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	let but2 = Button::builder()
		.label("Decrease")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	// let label = Label::builder().label("0").build();

	let number = Rc::new(Cell::new(0));

	// Connect to "clicked" signal of `button`
	button.connect_clicked(clone!(@weak number, @weak but2 => move |_| {
		// Set the label to "Hello World!" after the button has been clicked on
		// button.set_label("Hello World!");
		glib::spawn_future_local(async {
			let res = get_waiting_times().await;
			println!("{:?}", res.unwrap());
		});
		number.set(number.get() + 1);
		but2.set_label(&number.get().to_string());
	}));
	but2.connect_clicked(clone!(@weak button => move |_| {
		number.set(number.get() - 1);
		button.set_label(&number.get().to_string());
	}));

	let pane = Paned::builder()
		.orientation(gtk::Orientation::Vertical)
		.start_child(&button)
		.end_child(&but2)
		.build();

	// Create a window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("My GTK App")
		.child(&pane)
		.build();

	// Present window
	window.present();
}
