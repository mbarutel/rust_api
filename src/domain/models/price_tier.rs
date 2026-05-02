use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// This is not accurate at the moment. This still requires further thinking
// pub struct PriceTier {
//     pub id: u64,
//     pub conference_id: u64,
//     pub name: String,
//     pub description: Option<String>,
//     pub valid_from: Option<NaiveDateTime>,
//     pub valid_until: Option<NaiveDateTime>,
//     pub is_active: bool,
// }

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PriceTier {
    pub id: u64,
    pub conference_id: u64,
    pub price: Decimal,
    pub deadline: DateTime<Utc>,
}

pub fn generate_price_tiers(
    start_date: NaiveDate,
    walk_in_price: Decimal,
    steps: usize,
    discount_per_step: Decimal,
    is_perscent_discount: bool,
) -> Vec<PriceTier> {
    let walk_year = start_date.year();
    let walk_month = start_date.month() as i32;

    let cutoff_dates = {
        let mut dates: Vec<NaiveDate> = Vec::new();

        'outer: for m in 0..12 {
            let total_months = walk_year * 12 + (walk_month - 1) - m;
            let year = total_months / 12;
            let month = (total_months % 12 + 1) as u32;

            let mid = NaiveDate::from_ymd_opt(year, month, 15).unwrap();
            let end = last_day_of_month(year, month);

            for &d in &[end, mid] {
                if d < start_date {
                    dates.push(d);
                    if dates.len() >= steps {
                        break 'outer;
                    }
                }
            }
        }

        dates.sort();
        dates
    };

    let mut tiers: Vec<PriceTier> = cutoff_dates
        .iter()
        .enumerate()
        .map(|(idx, &date)| PriceTier {
            deadline: date.and_time(NaiveTime::MIN).and_utc(),
            price: calc_price(
                walk_in_price,
                steps,
                idx,
                discount_per_step,
                is_perscent_discount,
            ),
        })
        .collect();

    tiers.push(PriceTier {
        price: walk_in_price,
        deadline: start_date.and_time(NaiveTime::MIN).and_utc(),
    });

    tiers
}

fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year + 1, month + 1)
    };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap()
}

fn calc_price(
    walk_in_price: Decimal,
    steps: usize,
    idx: usize,
    discount_per_step: Decimal,
    is_perscent_discount: bool,
) -> Decimal {
    let price = if is_perscent_discount {
        let factor = Decimal::from(1)
            - (discount_per_step * Decimal::from(steps - idx) / Decimal::from(100));
        (walk_in_price * factor).round()
    } else {
        walk_in_price - (discount_per_step * Decimal::from(steps - idx))
    };

    price
}
