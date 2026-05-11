use chrono::{Datelike, Days, NaiveDate};
use rust_decimal::Decimal;

use crate::application::dto::CreatePriceTierRequest;

pub fn generate_price_tiers(start_date: NaiveDate) -> Vec<CreatePriceTierRequest> {
    const STEPS: usize = 8;
    const DISCOUNT_PER_STEP: u8 = 200;
    const WALK_IN_PRICE: usize = 2500;
    const IS_PERCENT_DISCOUNT: bool = false;

    let walk_year = start_date.year();
    let walk_month = start_date.month() as i32;
    let walk_in_price = Decimal::from(WALK_IN_PRICE);
    let discount_per_step = Decimal::from(DISCOUNT_PER_STEP);

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
                    if dates.len() >= STEPS {
                        break 'outer;
                    }
                }
            }
        }

        dates.sort();
        dates
    };

    let mut tiers: Vec<CreatePriceTierRequest> = cutoff_dates
        .iter()
        .enumerate()
        .map(|(idx, &date)| CreatePriceTierRequest {
            deadline: date,
            price: calc_price(
                walk_in_price,
                STEPS,
                idx,
                discount_per_step,
                IS_PERCENT_DISCOUNT,
            ),
        })
        .collect();

    tiers.push(CreatePriceTierRequest {
        price: Decimal::from(WALK_IN_PRICE),
        deadline: start_date,
    });

    tiers
}

fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
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
