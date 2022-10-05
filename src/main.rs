use macroquad::prelude::*;
use std::cell::RefCell;
use std::sync::mpsc;

#[macroquad::main("Train-Helper")]
async fn main() {
    let (sender, reciever) = mpsc::channel::<(RefCell<Vec<String>>, RefCell<Vec<String>>)>();

    std::thread::spawn(move || loop {
        sender
            .send(train_helper_lib::get_messages())
            .expect("Error sending data");
    });

    let (mut skm_messages, mut ztm_messages) = reciever.recv().expect("Sender hanged up");

    const FONT_SIZE: f32 = 30.0;
    loop {
        let mut text_position: f32 = FONT_SIZE;

        (skm_messages, ztm_messages) = match reciever.try_recv() {
            Ok((skm_m, ztm_m)) => (skm_m, ztm_m),
            Err(std::sync::mpsc::TryRecvError::Empty) => (skm_messages, ztm_messages),
            Err(_) => panic!("Communication error"),
        };

        clear_background(WHITE);
        skm_messages.borrow().iter().for_each(|x| {
            draw_text(&x, 20.0, text_position, FONT_SIZE, BLACK);
            text_position += FONT_SIZE;
        });
        ztm_messages.borrow().iter().for_each(|x| {
            draw_text(&x, 20.0, text_position, FONT_SIZE, BLACK);
            text_position += FONT_SIZE;
        });
        next_frame().await
    }
}
