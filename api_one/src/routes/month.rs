use std::fmt::format;
use actix_web::{get, web, App, HttpServer, Result, HttpResponse};

/**

{
    "dairy": {
        "date": "",
        "curr_day": "",
        "curr_month": "",
        "curr_year": "",
        "days": 31,
        "day_hours": [0:00, 01:00, 01:30 ...],
    },
    "events": [
        {
            "id": 1,
            "date": "",
            "details": "asdas asdas ad",
            "category": {
                "id": 1,
                "name": ""
            }
        }
    ],
    "symptoms": [
        {
            "id": 1,
            "date": "",
            "details": "asdas asdas ad",
            "symptom_type": {
                "id": 1,
                "name": ""
            }
        }
    ]
}
*/

#[get("/year/{year}/month/{month}")]
pub async fn month(year: String, month: String) -> String {
    "Hello".to_owned()
}