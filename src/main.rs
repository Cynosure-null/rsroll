use cursive::{views::{EditView, self}, view, CursiveRunnable};
#[allow(unused_imports)]

use cursive::{event::Key, menu, traits::*, views::Dialog, views::TextArea};
use std::{sync::atomic::{AtomicUsize, Ordering}, fmt::format};
use cursive::Cursive;
use std::thread;
mod calc;
mod music;

fn main() {
    let mut siv = cursive::default();

    let dice_arr = [100, 20, 12, 10, 8, 6, 4];

    let songs = music::index();

    siv.menubar()
       .add_subtree(
           "Dice",
           menu::Tree::new().with(move |tree|{
               for i in 0..dice_arr.len(){
                   let name = format!("d{}",dice_arr[i]);
                   tree.add_leaf(name, move |s|{
                       s.add_layer(
                       Dialog::new()
                               .title("How many would you like to roll?")
                               .content(EditView::new()
                                        .on_submit(move |s, text| {
                                            match text.parse::<i32>() {
                                                Ok(_n) => {
                                                    s.add_layer(
                                                        Dialog::info(
                                                            calc::roll(
                                                                _n, dice_arr[i]
                                                            ).to_string()))
                                                },
                                                Err(_e) => {
                                                    s.add_layer(
                                                        Dialog::info(
                                                            format!(
                                                                "Error: {}",
                                                                _e)
                                                        ))}

                                            }
                                        })
                               )
                       )
                   })
               }
           }))

       .add_subtree(
           "NPC",
           menu::Tree::new()
               .leaf("name", |s| {
                   s.add_layer(Dialog::info("This is never going to get made"))
               }))

       .add_subtree(
           "Music",
           menu::Tree::new()
               .with(move |tree| {
                   for i in songs{
                       let i_string = i.as_ref().unwrap().path().to_str().unwrap().to_string();
                       tree.add_leaf(i_string.clone(), move |s| {
                           let moveable_i = i_string.clone();
                           thread::spawn(move || {
                               music::play(moveable_i.clone());
                           });
                           s.add_layer(Dialog::info(format!("Now playing: {}", i_string.clone())));
                       })
                   }
               })
               .leaf("song1", |s| {
                   thread::spawn(move || {
                       music::play("test.ogg".to_string());
                   });
                   s.add_layer(Dialog::info("Now playing song1")
                   )
               }))

               .add_delimiter()
               .add_leaf("quit", |s| s.quit());


           siv.set_autohide_menu(false);

    siv.select_menubar();

    siv.add_global_callback(Key::Esc, |s| {
        s.select_menubar()
    });

    eprintln!("Here");

    siv.run();
}

fn ui_init()
{
}
