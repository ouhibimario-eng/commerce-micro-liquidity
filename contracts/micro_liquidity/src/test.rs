
use super::*;
use soroban_sdk::Env;

#[test]
fn test_initialize_and_read_configuration() {
    let env = Env::default();
    let contract_id = env.register(MicroLiquidityPool, ());
    let client = MicroLiquidityPoolClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let target: i128 = 1_000;

    env.mock_all_auths();

    client.initialize(&admin, &target);

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_funding_target(), target);
}
