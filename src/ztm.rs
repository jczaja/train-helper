pub mod ztm {
    use serde::{Deserialize, Serialize};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    // Example JSON
    //{
    //   "lastUpdate":"2022-09-07T06:58:03Z",
    //   "departures":[
    //
    //      {
    //         "id":"T32R158",
    //         "delayInSeconds":117,
    //         "estimatedTime":"2022-09-07T07:01:57Z",
    //         "headsign":"Stogi",
    //         "routeId":158,
    //         "scheduledTripStartTime":"2022-09-07T06:55:00Z",
    //         "tripId":32,
    //         "status":"REALTIME",
    //         "theoreticalTime":"2022-09-07T07:00:00Z",
    //         "timestamp":"2022-09-07T06:59:50Z",
    //         "trip":7206298,
    //         "vehicleCode":3029,
    //         "vehicleId":145789,
    //         "vehicleService":"158-01"
    //      },
    //      {
    //         "id":"T122R127",
    //         "delayInSeconds":645,
    //         "estimatedTime":"2022-09-07T07:06:45Z",
    //         "headsign":"Oliwa PKP",
    //         "routeId":127,
    //         "scheduledTripStartTime":"2022-09-07T06:20:00Z",
    //         "tripId":122,
    //         "status":"REALTIME",
    //         "theoreticalTime":"2022-09-07T06:56:00Z",
    //         "timestamp":"2022-09-07T06:59:50Z",
    //         "trip":7207719,
    //         "vehicleCode":2522,
    //         "vehicleId":188,
    //         "vehicleService":"127-02"
    //      }
    //    ]
    //}

    #[derive(Debug, Deserialize, Serialize)]
    struct Response {
        lastUpdate: String,
        departures: Vec<Departure>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Departure {
        id: String,
        delayInSeconds: Option<i32>,
        estimatedTime: String,
        headsign: String,
        routeId: u32,
    }

    pub struct ZTM<'a> {
        client: reqwest::Client,
        ztm_url: &'a str,
        proxy: Option<Vec<String>>,
        bus_stop_names: HashMap<&'a str, String>,
        departures: Vec<(&'a str, Vec<u32>, &'a str)>,
    }

    impl<'a> ZTM<'a> {
        fn get_bus_stop_names(
            departures: &Vec<(&'a str, Vec<u32>, &'a str)>,
        ) -> HashMap<&'a str, String> {
            let mut bus_stop_names: HashMap<&'a str, String> = HashMap::new();
            // TODO: Get actual mapping from bus stop number to its name
            bus_stop_names.insert("1752", "(Gdansk Zaspa SKM 01 -->): ".to_string());
            bus_stop_names.insert("1645", "(Gdansk Stadion 04 -->): ".to_string());
            bus_stop_names.insert("1988", "(Gdansk Stadion 06 -->): ".to_string());
            bus_stop_names.insert("2088", "(Gdansk Startowa 02 -->): ".to_string());
            bus_stop_names.insert("2075", "(Gdansk Mickiewicza 01 -->): ".to_string());
            bus_stop_names.insert("1768", "(Gdansk Hynka 01 -->): ".to_string());
            bus_stop_names.insert("1767", "(Gdansk Hynka 02 -->): ".to_string());
            bus_stop_names.insert("1482", "(Gdansk Dywizjonu 303 01 -->): ".to_string());
            bus_stop_names.insert("1485", "(Gdansk Pilotow 01 -->): ".to_string());
            bus_stop_names.insert("1404", "(Gdansk Firoga 01 -->): ".to_string());
            bus_stop_names.insert("1634", "(Gdansk Galeria Baltycka 07 -->): ".to_string());
            bus_stop_names
        }

        pub fn new(
            proxy: Option<Vec<String>>,
            departures: Vec<(&'a str, Vec<u32>, &'a str)>,
        ) -> Self {
            ZTM {
                // If there is proxy then pick first URL
                client: reqwest::Client::new(),
                proxy,
                ztm_url: "https://mapa.ztm.gda.pl/departures?stopId=",
                bus_stop_names: ZTM::get_bus_stop_names(&departures),
                departures,
            }
        }

        // Get message out of JSON based on given time
        fn get_message(
            &self,
            body: Response,
            bus_stop: &str,
            busses: &Vec<u32>,
            date_time: &chrono::DateTime<chrono::Local>,
        ) -> Vec<String> {
            let mut total_response: Vec<String> = vec![];
            busses.iter().for_each(|e| {
                let response = body.departures.iter().filter(|d| d.routeId == *e).fold(
                    "  ".to_string()
                        + &self.bus_stop_names[bus_stop]
                        + &e.to_string()
                        + " departs in:",
                    |response, d| {
                        // Compute estimated time of arrival in minutes
                        //"estimatedTime":"2022-09-07T07:01:57Z",
                        let mut estimation = chrono::DateTime::parse_from_str(
                            &(d.estimatedTime.clone() + " +0000"),
                            "%Y-%m-%dT%H:%M:%SZ %z",
                        )
                        .expect(&format!(
                            "Error parsing response lastupdate: {}",
                            &d.estimatedTime
                        ));

                        estimation = match d.delayInSeconds {
                            Some(secs) => estimation + chrono::Duration::seconds(secs as i64),
                            None => estimation,
                        };

                        let remaining_minutes =
                            estimation.signed_duration_since(*date_time).num_minutes();

                        response + &format!(" {} min,", remaining_minutes)
                    },
                );
                total_response.push(response);
            });
            total_response
        }

        async fn get_info(
            &self,
            bus_stop: &str,
            busses: &Vec<u32>,
            msg_prefix: &str,
            order_number: u32,
            messages: &Rc<RefCell<Vec<(String, u32)>>>,
        ) {
            let res = self
                .client
                .get(&(self.ztm_url.to_string() + bus_stop))
                .send()
                .await
                .expect("Error: fetching ZTM data");

            let msgs = self.get_message(
                res.json::<Response>()
                    .await
                    .expect("Error converting response to JSON in ZTM"),
                bus_stop,
                busses,
                &chrono::Local::now(),
            );
            let scale = 6;
            messages
                .borrow_mut()
                .push((msg_prefix.to_string(), order_number << scale));
            let mut ordered_msgs: Vec<(String, u32)> = vec![];
            let mut i: u32 = 0;
            for msg in msgs {
                ordered_msgs.push((msg, order_number << scale + i));
                i += i;
            }
            messages.borrow_mut().append(&mut ordered_msgs);
        }

        pub async fn submit(&self) -> Result<Rc<RefCell<Vec<(String, u32)>>>, String> {
            let messages = Rc::new(RefCell::new(vec![]));
            let mut myfutures: Vec<_> = Vec::new();

            let mut i = 0;
            for (bus_stop, busses, msg_prefix) in &self.departures {
                myfutures.push(self.get_info(bus_stop, busses, msg_prefix, i, &messages));
                i += 1;
            }

            futures::future::join_all(myfutures).await;

            Ok(messages)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::prelude::*;

        type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
        type GenericResult<T> = Result<T, GenericError>;

        //  https://mapa.ztm.gda.pl/departures?stopId=1752
        #[test]
        fn test_ztm() -> Result<(), String> {
            let ztm = ZTM::new(
                None,
                vec![(
                    "1752", // ID of bus stop
                    vec![],
                    "Bus to Arena ",
                )],
            );

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Error: Unable to init runtime");
            rt.block_on(ztm.submit())?;
            Ok(())
        }

        #[test]
        fn test_parsing_ztm_message() -> GenericResult<()> {
            // Let's read data to parse from stored file
            let mut file = std::fs::File::open("data/test_ztm_data.txt")?;

            let mut s = String::new();
            file.read_to_string(&mut s)?;

            let s: Response = serde_json::from_str(&s)?;

            let date_str = "2022-09-07 07:25 +0000";
            let datetime = chrono::DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M %z").unwrap();

            let response = ZTM::new(
                None,
                vec![(
                    "1752", // ID of bus stop
                    vec![158, 127],
                    "Bus to Arena ",
                )],
            )
            .get_message(
                s,
                "1752",
                &vec![158, 127],
                &chrono::DateTime::from(datetime),
            );
            let expected_response: Vec<String> = vec![
                "  (Gdansk Zaspa SKM 01 -->): 158 departs in: 18 min, 55 min,".to_owned(),
                "  (Gdansk Zaspa SKM 01 -->): 127 departs in: 19 min, 30 min, 48 min, 68 min, 88 min,"
                    .to_owned(),
            ];

            assert_eq!(response, expected_response);
            Ok(())
        }
    }
}
