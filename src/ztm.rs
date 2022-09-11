pub mod ztm {
    use serde::{Deserialize, Serialize};

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

    pub struct ZTM {
        ztm_url: String,
        proxy: Option<Vec<String>>,
        busses: Vec<u32>,
    }

    impl ZTM {
        //  https://mapa.ztm.gda.pl/departures?stopId=1752
        pub fn new(proxy: Option<Vec<String>>, from: &str, busses: Vec<u32>) -> Self {
            ZTM {
                ztm_url: "https://mapa.ztm.gda.pl/departures?stopId=".to_string() + from,
                proxy: proxy,
                busses: busses,
            }
        }

        // Get message out of JSON based on given time
        fn get_message(
            &self,
            body: Response,
            date_time: &chrono::DateTime<chrono::Local>,
        ) -> String {
            let mut total_response = "".to_owned();
            self.busses.iter().for_each(|e| {
                let response = body.departures.iter().filter(|d| d.routeId == *e).fold(
                    e.to_string() + ":",
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

                        estimation = match (d.delayInSeconds) {
                            Some(secs) => estimation + chrono::Duration::seconds(secs as i64),
                            None => estimation,
                        };

                        let remaining_minutes =
                            estimation.signed_duration_since(*date_time).num_minutes();

                        response + &format!(" {} min,", remaining_minutes)
                    },
                );
                total_response += &(format!("{}\n", response));
            });
            total_response
        }

        pub fn submit(&self) -> Result<String, String> {
            // If there is proxy then pick first URL
            let client = reqwest::blocking::Client::new();

            // Get IDs of stations e.g. Gdansk Wrzeszcz : 7534
            let res = client.get(&(self.ztm_url.clone())).send();

            // HERE is fine to return
            // Returning here is fine
            let res = match res {
                Ok(result) => result.text(),
                Err(i) => return Err(format!("Error sending ZTM request: {}", i)),
            };

            let actual_response = res.expect("Error: unwrapping ZTM response");
            // Get Data

            // Send a request to ZTM web page
            let request = &self.ztm_url;

            // Get actual times for our chosen destination
            let mut res = client
                .get(request)
                .send()
                .expect("Error sending ZTM request");

            // Lets get current data and time

            let message = self.get_message(
                res.json::<Response>()
                    .expect("Error converting response to JSON in ZTM"),
                &chrono::Local::now(),
            );
            Ok(message)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json::json;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
        type GenericResult<T> = Result<T, GenericError>;

        //  https://mapa.ztm.gda.pl/departures?stopId=1752
        #[test]
        fn test_ztm() -> Result<(), String> {
            let ztm = ZTM::new(
                None,
                "1752", // ID of bus stop
                vec![],
            )
            .submit()?;
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
                "1752",         // ID of bus stop
                vec![158, 127], // bus numbers we are interested in
            )
            .get_message(s, &chrono::DateTime::from(datetime));
            let expected_response =
                "158: 18 min, 55 min,\n127: 19 min, 30 min, 48 min, 68 min, 88 min,\n";
            assert_eq!(response, expected_response);
            Ok(())
        }
    }
}
