use csv::ReaderBuilder;
use serde;

#[derive(Debug, serde::Deserialize)]
pub struct OriginalRecord {
    // boekingsdatum: String,
    valutadatum: String,
    tegenpartij: String,
    mededeling: String,
    bedrag: f64,
}

pub fn parse_csv(text: String) -> Vec<OriginalRecord> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(text.as_bytes());

    reader
        .records()
        .map(|record| {
            let record = record.unwrap();
            let bedrag = record.get(10).unwrap();
            let bedrag = bedrag.replace(",", ".");
            let bedrag = bedrag.parse::<f64>().ok().unwrap();
            OriginalRecord {
                // boekingsdatum: record.get(1).unwrap().to_string(),
                valutadatum: record.get(9).unwrap().to_string(),
                tegenpartij: record.get(5).unwrap().to_string(),
                mededeling: record.get(14).unwrap().to_string(),
                bedrag,
            }
        })
        .collect()
}

pub fn to_ynab_format(records: Vec<OriginalRecord>) -> String {
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b',')
        .from_writer(vec![]);

    // Writer headers
    writer
        .write_record(&["Date", "Payee", "Memo", "Amount"])
        .unwrap();

    // Write records
    for record in records {
        writer
            .write_record(&[
                record.valutadatum,
                record.tegenpartij,
                record.mededeling,
                record.bedrag.to_string(),
            ])
            .unwrap();
    }

    let csv = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    csv
}
