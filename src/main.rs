use macroquad::prelude::*;
use macroquad::ui::root_ui;

mod skm;
mod ztm;

#[macroquad::main("TrainHelper")]
async fn main() {
    let messages = skm::skm::SKM::new(
        "https://skm.trojmiasto.pl/".to_string(),
        None,
        vec![
            (
                vec![
                    "Gdansk Wrzeszcz".to_string(),
                    "Gdansk Port Lotniczy".to_string(),
                ],
                format!("Train to work "),
            ),
            (
                vec![
                    "Gdansk Port Lotniczy".to_string(),
                    "Gdansk Wrzeszcz".to_string(),
                ],
                format!("Train home from work"),
            ),
            (
                vec!["Gdansk Zaspa".to_string(), "Sopot".to_string()],
                format!("Train to Sopot "),
            ),
            (
                vec!["Sopot".to_string(), "Gdansk Zaspa".to_string()],
                format!("Train home from Sopot "),
            ),
            (
                vec!["Gdansk Zaspa".to_string(), "Gdansk Glowny".to_string()],
                format!("Train to Gdansk "),
            ),
            (
                vec!["Gdansk Glowny".to_string(), "Gdansk Zaspa".to_string()],
                format!("Train home from Gdansk "),
            ),
        ],
    )
    .submit();

    // busses
    let to_arena = ztm::ztm::ZTM::new(
        None,
        "1752", // ID of bus stop
        vec![158],
    )
    .submit();
    let to_arena_message = match to_arena {
        Ok(mesg) => format!("Bus to PARKOUR (GDANSK ZASPA SKM 01 -->):\n {}", mesg),
        Err(err_msg) => err_msg,
    };

    let mut myskin = root_ui().default_skin().clone();
    myskin.label_style = root_ui().style_builder().font_size(35).build();
    myskin.button_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(100, 100, 255, 255))
        .font_size(80)
        .build();
    root_ui().push_skin(&myskin);
    let mut running = true;
    let skm_messages = match messages {
        Ok(msgs) => msgs,
        Err(err_msg) => vec![err_msg],
    };

    while running {
        clear_background(WHITE);

        skm_messages.iter().for_each(|x| root_ui().label(None, &x));
        root_ui().label(None, &to_arena_message);
        if root_ui().button(None, "Exit") {
            println!("pushed");
            running = false;
        }
        next_frame().await
    }
    root_ui().pop_skin();
}
