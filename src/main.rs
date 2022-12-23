use std::sync::mpsc;

fn main() {
    let (sender, reciever) = mpsc::channel::<(Vec<String>, Vec<String>)>();

    std::thread::spawn(move || loop {
        sender
            .send(train_helper_lib::get_messages())
            .expect("Error sending data");
    });

    let (skm_messages, _ztm_messages) = reciever.recv().expect("Sender hanged up");

    skm_messages.iter().for_each(|x| {
        println!("{}", &x);
    });

    //        ztm_messages.iter().for_each(|x| {
    //            draw_text(&x, 20.0, text_position, FONT_SIZE, BLACK);
    //        });
}
