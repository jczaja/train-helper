mod skm;
mod ztm;
use std::cell::RefCell;
use std::rc::Rc;

async fn get_requests() -> (Result<Rc<RefCell<Vec<String>>>, String>, Result<Vec<String>, String>)
{
    let try_skm_messages = skm::skm::SKM::new(
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
                vec![
                    "Gdansk Port Lotniczy".to_string(),
                    "Gdynia Glowna".to_string(),
                ],
                format!("Train to Gdynia Glowna:"),
            ),
            (
                vec![
                    "Gdansk Port Lotniczy".to_string(),
                    "Koscierzyna".to_string(),
                ],
                format!("Train to Koscierzyna:"),
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
    );

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
            (
                "1404", // ID of bus stop
                vec![110],
                "Bus to Gdansk Wrzeszcz (Through IKEA):\n",
            ),
            (
                "1404", // ID of bus stop
                vec![210],
                "Bus to Gdansk Glowny (Through IKEA):\n",
            ),
            (
                "1404", // ID of bus stop
                vec![122],
                "Bus to Sopot (Through IKEA):\n",
            ),
        ],
    )
    .submit();

    (try_skm_messages.submit().await, try_ztm_messages)
}



pub fn get_messages() -> (RefCell<Vec<String>>, Vec<String>) {

    let (try_skm_messages,try_ztm_messages) = futures::executor::block_on(get_requests());

    let skm_messages = match try_skm_messages {
        Ok(msgs) => msgs,
        Err(err_msg) => Rc::new(RefCell::new(vec![err_msg])),
    };

    let ztm_messages = match try_ztm_messages {
        Ok(msgs) => msgs,
        Err(err_msg) => vec![err_msg],
    };

    (((*skm_messages).clone()), ztm_messages)
}
