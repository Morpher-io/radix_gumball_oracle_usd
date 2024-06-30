use std::str::FromStr;

use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone)]
pub struct PriceMessage {
    pub market_id: String,
    pub price: Decimal,
    pub nonce: u64,
    pub created_at: u64,
}

impl PriceMessage {
    pub fn to_string(&self) -> String {
        return format!(
            "{}-{}-{}-{}",
            self.market_id, self.price, self.nonce, self.created_at
        );
    }
}

impl FromStr for PriceMessage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_string = "Failed to parse input message".to_string();
        let parts: Vec<&str> = s.split("-").collect();

        if parts.len() != 4 {
            Err(err_string)
        } else {
            let market_id = parts
                .first()
                .unwrap()
                .parse()
                .map_err(|_| "Could not parse the market id".to_string())?;

            let price = Decimal::from_str(parts.get(1).unwrap())
                .map_err(|_| "Could not parse the price".to_string())?;

            let nonce = parts
                .get(2)
                .unwrap()
                .parse()
                .map_err(|_| "Could not parse the nonce".to_string())?;

            let created_at = parts
                .get(3)
                .unwrap()
                .parse()
                .map_err(|_| "Could not parse the creation date".to_string())?;

            Ok(PriceMessage {
                market_id,
                price,
                nonce,
                created_at,
            })
        }
    }
}

#[cfg(test)]
mod price_message_tests {
    use scrypto::prelude::*;

    use crate::price_message::PriceMessage;

    #[test]
    pub fn test_to_string() {
        let price_message = PriceMessage {
            market_id: "TEST:MARKET",
            price: dec!(1000.234),
            nonce: 1,
            created_at: 1230,
        };

        assert_eq!(price_message.to_string(), "TEST:MARKET-1000.234-1-1230");
    }

    #[test]
    pub fn from_string_test() {
        let price_message = PriceMessage::from_str("TEST:MARKET-1000.234-1-1230").unwrap();
        assert!(
            price_message.market_id == "TEST:MARKET"
                && price_message.price == dec!(1000.234)
                && price_message.nonce == 1
                && price_message.created_at == 1230
        );

        assert!(PriceMessage::from_str("TEST-1000.234-1-1230-5").is_err())
    }
}
