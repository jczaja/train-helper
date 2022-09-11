use macroquad::prelude::*;
use macroquad::ui::root_ui;

mod skm;
mod ztm;

#[macroquad::main("TrainHelper")]
async fn main() {
    let to_work_message = skm::skm::SKM::new(
        "https://skm.trojmiasto.pl/".to_string(),
        None,
        vec![
            "Gdansk Wrzeszcz".to_string(),
            "Gdansk Port Lotniczy".to_string(),
        ],
    )
    .submit();
    let to_work_message = match to_work_message {
        Ok(mesg) => format!("Train to work {}", mesg),
        Err(err_msg) => err_msg,
    };
    let home_message = skm::skm::SKM::new(
        "https://skm.trojmiasto.pl/".to_string(),
        None,
        vec![
            "Gdansk Port Lotniczy".to_string(),
            "Gdansk Wrzeszcz".to_string(),
        ],
    )
    .submit();
    let home_message = match home_message {
        Ok(mesg) => format!("Train home {}", mesg),
        Err(err_msg) => err_msg,
    };

    // busses    
    let to_arena = ztm::ztm::ZTM::new(
        None,
        "1752", // ID of bus stop
        vec![158],
    )
    .submit();
    let to_arena_message = match to_arena {
        Ok(mesg) => format!("Busses to PARKOUR (GDANSK ZASPA SKM 01 -->):\n {}", mesg),
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
    while running {
        clear_background(WHITE);

        root_ui().label(None, &to_work_message);
        root_ui().label(None, &home_message);
        root_ui().label(None, &to_arena_message);
        if root_ui().button(None, "Exit") {
            println!("pushed");
            running = false;
        }
        next_frame().await
    }
    root_ui().pop_skin();
}
