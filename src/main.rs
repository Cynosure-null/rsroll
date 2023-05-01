use cursive::{views::{EditView, self}, view, CursiveRunnable};
#[allow(unused_imports)]

use cursive::{event::Key, menu, traits::*, views::Dialog, views::TextArea};
use std::sync::atomic::{AtomicUsize, Ordering};
use cursive::Cursive;
mod calc;

fn main() {
    let mut siv = cursive::default();

    siv.menubar()
       .add_subtree(
           "Dice",
           menu::Tree::new()
               .leaf("d100", |s| {
                   s.add_layer(
                       Dialog::new()
                           .title("How many would you like to roll?")
                           .content(
                               EditView::new()
                                   .on_submit(move |s, text| {
                                       match text.parse::<i32>() {
                                           Ok(n) => {
                                               s.add_layer(Dialog::info(calc::roll(n, 100).to_string()));
                                           },
                                           Err(e) => {s.add_layer(Dialog::info(
                                               "Error: input not an integer"
                                           ));},
                                       }
                                   })
                           ))})

               .leaf("d20", |s| {
                   s.add_layer(
                       Dialog::new()
                           .title("How many would you like to roll?")
                           .content(
                               EditView::new()
                                   .on_submit(move |s, text| {
                                       match text.parse::<i32>() {
                                           Ok(n) => {
                                               s.add_layer(Dialog::info(calc::roll(n, 20).to_string()));
                                           },
                                           Err(e) => {s.add_layer(Dialog::info(
                                               "Error: input not an integer"
                                           ));},
                                       }
                                   })
                           ))})
               .leaf("d12", |s| {
                   s.add_layer(
                       Dialog::new()
                           .title("How many would you like to roll?")
                           .content(
                               EditView::new()
                                   .on_submit(move |s, text| {
                                       match text.parse::<i32>() {
                                           Ok(n) => {
                                               s.add_layer(Dialog::info(calc::roll(n, 12).to_string()));
                                           },
                                           Err(e) => {s.add_layer(Dialog::info(
                                               "Error: input not an integer"
                                           ));},
                                       }
                                   })
                           ))})

               .leaf("d10", |s| {
                   s.add_layer(
                       Dialog::new()
                           .title("How many would you like to roll?")
                           .content(
                               EditView::new()
                                   .on_submit(move |s, text| {
                                       match text.parse::<i32>() {
                                           Ok(n) => {
                                               s.add_layer(Dialog::info(calc::roll(n, 10).to_string()));
                                           },
                                           Err(e) => {s.add_layer(Dialog::info(
                                               "Error: input not an integer"
                                           ));},
                                       }
                                   })
                           ))})
               .leaf("d8", |s| {
                   s.add_layer(
                       Dialog::new()
                           .title("How many would you like to roll?")
                           .content(
                               EditView::new()
                                   .on_submit(move |s, text| {
                                       match text.parse::<i32>() {
                                           Ok(n) => {
                                               s.add_layer(Dialog::info(calc::roll(n, 8).to_string()));
                                           },
                                           Err(e) => {s.add_layer(Dialog::info(
                                               "Error: input not an integer"
                                           ));},
                                       }
                                   })
                           ))})
               .leaf("d6", |s| {
                   s.add_layer(
                        Dialog::new()
                            .title("How many would you like to roll?")
                            .content(
                                EditView::new()
                                    .on_submit(move |s, text| {
                                        match text.parse::<i32>() {
                                            Ok(n) => {
                                                s.add_layer(Dialog::info(calc::roll(n, 6).to_string()));
                                            },
                                            Err(e) => {s.add_layer(Dialog::info(
                                                "Error: input not an integer"
                                            ));},
                                        }
                                    })
                            ))})

                .leaf("d4", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("How many would you like to roll?")
                            .content(
                                EditView::new()
                                    .on_submit(move |s, text| {
                                        match text.parse::<i32>() {
                                            Ok(n) => {
                                                s.add_layer(Dialog::info(calc::roll(n, 4).to_string()));
                                            },
                                            Err(e) => {s.add_layer(Dialog::info(
                                                "Error: input not an integer"
                                            ));},
                                        }
                                    })
                            ))}))

       .add_subtree(
           "NPC",
           menu::Tree::new()
               .leaf("name", |s| {
                   s.add_layer(Dialog::info("This is never going to get made"))
               }))

       .add_subtree(
           "Music",
           menu::Tree::new()
               .leaf("song1", |s| {
                   s.add_layer(Dialog::info("Now play a song"))
               }))

       .add_delimiter()
       .add_leaf("quit", |s| s.quit());

    // When `autohide` is on (default), the menu only appears when active.
    // Turning it off will leave the menu always visible.
    // Try uncommenting this line!

    // siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}

fn ui_init()
{
}
