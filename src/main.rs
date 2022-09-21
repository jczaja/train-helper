use macroquad::prelude::*;
use std::sync::mpsc;

mod skm;
mod ztm;

fn get_messages(sender: mpsc::Sender<(Vec<String>, Vec<String>)>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        loop {
            let messages = skm::skm::SKM::new(
                "https://skm.trojmiasto.pl/".to_string(),
                None,
                vec![
                    (
                        vec![
                            "Gdansk Wrzeszcz".to_string(),
                            "Gdansk Port Lotniczy".to_string(),
                        ],
                        format!("Train to work: "),
                    ),
                    (
                        vec![
                            "Gdansk Port Lotniczy".to_string(),
                            "Gdansk Wrzeszcz".to_string(),
                        ],
                        format!("Train home from work:"),
                    ),
                    (
                        vec!["Gdansk Zaspa".to_string(), "Sopot".to_string()],
                        format!("Train to Sopot:"),
                    ),
                    (
                        vec!["Sopot".to_string(), "Gdansk Zaspa".to_string()],
                        format!("Train home from Sopot:"),
                    ),
                    (
                        vec!["Gdansk Zaspa".to_string(), "Gdansk Glowny".to_string()],
                        format!("Train to Gdansk:"),
                    ),
                    (
                        vec!["Gdansk Glowny".to_string(), "Gdansk Zaspa".to_string()],
                        format!("Train home from Gdansk:"),
                    ),
                ],
            )
            .submit();

            // busses
            let try_ztm_messages = ztm::ztm::ZTM::new(
                None,
                vec![
                    (
                        "1482", // ID of bus stop
                        vec![158, 258],
                        "Bus to Parkour:\n",
                    ),
                    (
                        "1645", // ID of bus stop
                        vec![158],
                        "Bus home from Parkour:\n",
                    ),
                    (
                        "1768", // ID of bus stop
                        vec![227],
                        "Bus to Jelitkowo:\n",
                    ),
                    (
                        "1767", // ID of bus stop
                        vec![227],
                        "Bus to Galeria Baltycka:\n",
                    ),
                    (
                        "2088", // ID of bus stop
                        vec![2, 4, 8],
                        "Tram to Mickiewicza:\n",
                    ),
                    (
                        "2075", // ID of bus stop
                        vec![2, 4, 8],
                        "Tram home from Mickiewicza:\n",
                    ),
                ],
            )
            .submit();

            let skm_messages = match messages {
                Ok(msgs) => msgs,
                Err(err_msg) => vec![err_msg],
            };

            let ztm_messages = match try_ztm_messages {
                Ok(msgs) => msgs,
                Err(err_msg) => vec![err_msg],
            };

            sender
                .send((skm_messages, ztm_messages))
                .expect("Error sending data");
        }
    })
}

#[macroquad::main("TrainHelper")]
async fn main() {
    let (sender, reciever) = mpsc::channel::<(Vec<String>, Vec<String>)>();

    get_messages(sender);

    let (mut skm_messages, mut ztm_messages) = reciever.recv().expect("Sender hanged up");
    const FONT_SIZE: f32 = 30.0;
    loop {
        let mut text_position: f32 = FONT_SIZE;

        (skm_messages, ztm_messages) = match (reciever.try_recv()) {
            Ok((skm_m, ztm_m)) => (skm_m, ztm_m),
            Err(std::sync::mpsc::TryRecvError::Empty) => (skm_messages, ztm_messages),
            Err(_) => panic!("Communication error"),
        };

        clear_background(WHITE);
        skm_messages.iter().for_each(|x| {
            draw_text(&x, 20.0, text_position, FONT_SIZE, BLACK);
            text_position += FONT_SIZE
        });
        ztm_messages.iter().for_each(|x| {
            draw_text(&x, 20.0, text_position, FONT_SIZE, BLACK);
            text_position += FONT_SIZE
        });
        next_frame().await
    }
}
