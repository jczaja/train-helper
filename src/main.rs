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
                "1752", // ID of bus stop
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

    let mut myskin = root_ui().default_skin().clone();
    myskin.label_style = root_ui().style_builder().font_size(35).build();
    myskin.button_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(100, 100, 255, 255))
        .font_size(80)
        .build();
    root_ui().push_skin(&myskin);
    let skm_messages = match messages {
        Ok(msgs) => msgs,
        Err(err_msg) => vec![err_msg],
    };

    let ztm_messages = match try_ztm_messages {
        Ok(msgs) => msgs,
        Err(err_msg) => vec![err_msg],
    };

    const FONT_SIZE: f32 = 30.0;
    loop {
        let mut text_position: f32 = FONT_SIZE;
        clear_background(WHITE);
        //skm_messages.iter().for_each(|x| root_ui().label(None, &x));
        //ztm_messages.iter().for_each(|x| root_ui().label(None, &x));
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
