#[cfg(test)]
mod trading_test {
    use test_engine::prelude::*;

    use oracle_signature::oracle::OracleSubscription;
    use oracle_signature::price_message::PriceMessage;

    global_package!(TRADING, ".");

    pub fn sign(message: PriceMessage, secret_key: &Bls12381G1PrivateKey) -> String {
        secret_key
            .sign_v1(message.to_string().as_bytes())
            .to_string()
    }

    pub fn instantiate_oracle() -> (TestEngine, Bls12381G1PrivateKey) {
        let mut test_engine = TestEngine::new();

        let secret_key = Bls12381G1PrivateKey::from_u64(1).unwrap();

        test_engine.add_global_package("morpher package", &TRADING);

        test_engine.new_component(
            "morpher oracle",
            "MorpherOracle",
            "instantiate",
            env_args!(
                secret_key.public_key().to_string(),
                dec!("30"),
                Environment::Account("default")
            ),
        );

        test_engine.call_faucet();

        (test_engine, secret_key)
    }

    #[test]
    fn test_new_subscription() {
        let (mut test_engine, _) = instantiate_oracle();

        let xrd_before = test_engine.current_balance("XRD");

        test_engine
            .call_method_builder(
                "new_subscription",
                env_args!(6u64, Fungible::Bucket("XRD", 190)),
            )
            .output("rtm/", "new_subscription")
            .execute()
            .assert_is_success();

        let xrd_after = test_engine.current_balance("XRD");

        // Check that only 180 XRD have been paid for 6 months
        assert_eq!(xrd_before - xrd_after, dec!(180));

        // Check that we received a subscription nft
        let ids = test_engine.current_ids_balance("morpher subscription");

        assert_eq!(ids.len(), 1);

        // Check that it has the right expiration time
        let data: OracleSubscription =
            test_engine.get_non_fungible_data("Morpher subscription", ids.first().unwrap().clone());

        assert_eq!(data.expiration_time, 6 * 2_592_000);
    }

    #[test]
    fn test_new_zero_month_subscription_fails() {
        let (mut test_engine, _) = instantiate_oracle();

        test_engine
            .call_method(
                "new_subscription",
                env_args!(0u64, Fungible::Bucket("XRD", 190)),
            )
            .assert_failed_with("Cannot add 0 months to the subscription!");
    }

    #[test]
    fn test_new_subscription_with_other_token_fails() {
        let (mut test_engine, _) = instantiate_oracle();

        test_engine.new_token("test token", 1000);
        test_engine
            .call_method(
                "new_subscription",
                env_args!(6u64, Fungible::Bucket("test token", 190)),
            )
            .assert_failed_with("The payment should be made in XRD tokens!");
    }

    #[test]
    fn test_new_subscription_with_not_enough_tokens_fails() {
        let (mut test_engine, _) = instantiate_oracle();

        test_engine
            .call_method(
                "new_subscription",
                env_args!(6u64, Fungible::Bucket("XRD", 170)),
            )
            .assert_failed_with(
                "For 6 months, the expected payment is 180 XRD (only supplied 170)",
            );
    }

    pub fn instantiate_with_subscription() -> (TestEngine, Bls12381G1PrivateKey) {
        let (mut test_engine, secret_key) = instantiate_oracle();

        test_engine
            .call_method_builder(
                "new_subscription",
                env_args!(6u64, Fungible::Bucket("XRD", 190)),
            )
            .output("rtm/", "new_subscription")
            .execute()
            .assert_is_success();

        (test_engine, secret_key)
    }

    #[test]
    fn test_renew_subscription() {
        let (mut test_engine, _) = instantiate_with_subscription();

        let subscription_id = test_engine.current_ids_balance("Morpher subscription");

        test_engine
            .call_method_builder(
                "renew_subscription",
                env_args!(
                    subscription_id.clone().first().unwrap().clone(),
                    1u64,
                    Fungible::Bucket("XRD", 30)
                ),
            )
            .output("rtm/", "renew_subscription")
            .execute()
            .assert_is_success();

        let data: OracleSubscription = test_engine.get_non_fungible_data(
            "Morpher subscription",
            subscription_id.first().unwrap().clone(),
        );

        assert_eq!(data.expiration_time, 7 * 2_592_000);
    }

    #[test]
    fn test_renew_subscription_after_expiry() {
        let (mut test_engine, _) = instantiate_with_subscription();

        let subscription_id = test_engine.current_ids_balance("Morpher subscription");

        test_engine.advance_time(7 * 2_592_000);

        test_engine
            .call_method(
                "renew_subscription",
                env_args!(
                    subscription_id.clone().first().unwrap().clone(),
                    1u64,
                    Fungible::Bucket("XRD", 30)
                ),
            )
            .assert_is_success();

        let data: OracleSubscription = test_engine.get_non_fungible_data(
            "Morpher subscription",
            subscription_id.first().unwrap().clone(),
        );

        assert_eq!(data.expiration_time, 8 * 2_592_000);
    }

    pub fn instantiate_with_trading() -> (TestEngine, Bls12381G1PrivateKey) {
        let (mut test_engine, secret_key) = instantiate_with_subscription();

        let ids = test_engine.current_ids_balance("Morpher subscription");
        test_engine.new_component(
            "trading component",
            "TradingComponent",
            "instantiate",
            env_args!(
                30i64,
                Environment::Component("Morpher Oracle"),
                NonFungible::Bucket("Morpher subscription", ids),
                Environment::Account("default")
            ),
        );

        test_engine.set_current_component("trading component");

        (test_engine, secret_key)
    }

    #[test]
    fn test_buy() {
        let (mut test_engine, secret_key) = instantiate_with_trading();

        let price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 0,
            created_at: 0,
        };

        test_engine
            .call_method_builder(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message, &secret_key)
                ),
            )
            .output("rtm/", "buy")
            .execute()
            .assert_is_success();

        assert_eq!(test_engine.current_balance("MPH"), dec!(3))
    }

    #[test]
    fn test_buy_with_expired_subscription_fails() {
        let (mut test_engine, secret_key) = instantiate_with_trading();
        test_engine.advance_time(2_592_000 * 6 + 1);
        let price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 0,
            created_at: 0,
        };

        test_engine
            .call_method(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message, &secret_key)
                ),
            )
            .assert_failed_with("Subscription has expired!");
    }

    #[test]
    fn test_buy_fails_wrong_signature() {
        let (mut test_engine, _) = instantiate_with_trading();

        let secret_key = Bls12381G1PrivateKey::from_u64(2).unwrap();

        let price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 0,
            created_at: 0,
        };

        test_engine
            .call_method(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message, &secret_key)
                ),
            )
            .assert_failed_with("Verification of signature failed!");
    }

    #[test]
    fn test_buy_fails_price_out_of_date() {
        let (mut test_engine, secret_key) = instantiate_with_trading();

        test_engine.advance_time(32);

        let price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 0,
            created_at: 0,
        };

        test_engine
            .call_method(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message, &secret_key)
                ),
            )
            .assert_failed_with("This price is out of date!");
    }

    #[test]
    fn test_buy_already_used_nonce_fails() {
        let (mut test_engine, secret_key) = instantiate_with_trading();
        let price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 0,
            created_at: 0,
        };

        test_engine
            .call_method(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message.clone(), &secret_key)
                ),
            )
            .assert_is_success();

        test_engine
            .call_method(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message, &secret_key)
                ),
            )
            .assert_failed_with("This nonce has already been used");
    }

    #[test]
    fn test_sell() {
        let (mut test_engine, secret_key) = instantiate_with_trading();
        let price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 0,
            created_at: 0,
        };

        test_engine
            .call_method_builder(
                "buy",
                env_args!(
                    dec!(3),
                    Fungible::Bucket("XRD", 3),
                    price_message.to_string(),
                    sign(price_message, &secret_key)
                ),
            )
            .output("rtm/", "sell")
            .execute()
            .assert_is_success();

        let new_price_message = PriceMessage {
            market_id: 0,
            price: dec!(1),
            nonce: 1,
            created_at: 0,
        };

        let xrd_before = test_engine.current_balance("xrd");

        test_engine
            .call_method(
                "sell",
                env_args!(
                    FungibleAll::FromAccount("MPH"),
                    new_price_message.to_string(),
                    sign(new_price_message, &secret_key)
                ),
            )
            .assert_is_success();

        let xrd_after = test_engine.current_balance("xrd");

        assert_eq!(xrd_after - xrd_before, dec!(3));
        assert_eq!(test_engine.current_balance("mph"), dec!(0))
    }

    #[test]
    fn with_backend_keys() {
        let mut test_engine = TestEngine::new();

        let secret_key = Bls12381G1PrivateKey::from_bytes(&[
            103, 213, 63, 23, 11, 144, 140, 171, 185, 235, 50, 108, 60, 51, 119, 98, 213, 146, 137,
            168, 254, 199, 159, 123, 201, 37, 75, 88, 75, 115, 38, 92,
        ])
        .unwrap();

        test_engine.add_global_package("morpher package", &TRADING);

        test_engine.new_component(
            "morpher oracle",
            "MorpherOracle",
            "instantiate",
            env_args!(
                secret_key.public_key().to_string(),
                dec!("30"),
                Environment::Account("default")
            ),
        );
    }
}
