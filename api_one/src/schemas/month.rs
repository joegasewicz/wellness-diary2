use serde::Serialize;

enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    November,
    December,
}


struct Year {

}



#[derive(Serialize)]
struct Period {
    curr_year: u16,
    curr_month: String,

}

