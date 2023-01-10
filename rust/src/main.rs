use std::env;
use std::str::FromStr;
use std::time::Duration;

use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let http = web3::transports::Http::new(&env::var("RPC_ENDPOINT").unwrap()).unwrap();
    let web3s = web3::Web3::new(http);

    let  accounts = web3s.eth().accounts().await?;
    // accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    // println!("Accounts: {:?}", accounts);

    let owner = accounts[0];
    let recipient = accounts[1];

    let aave_addr = Address::from_str(&env::var("CONTRACT_ADDR").unwrap()).unwrap();
    let token_contract =
        Contract::from_json(web3s.eth(), aave_addr, include_bytes!("abi.json")).unwrap();

    let cloned_token = token_contract.clone();

    let handle = tokio::spawn(async move {
        // check point of second account
        loop {
            let point: U256 = cloned_token.query("getPoint", (), recipient, Options::default(), None).await.unwrap();

            println!("Current point is {}", point);

            if point.gt(&U256::from(0)) {
                println!("Point is added. Terminate thread.");
                break;
            }

            sleep_ms(700).await;
        }
    });
            
    // add point to second account
    sleep_ms(5000).await;

    let amount = U256::from(100);

    println!("Adding {} points to {}", amount, recipient);
    token_contract.call("addPoint", (recipient, amount), owner, Options::default()).await.unwrap();

    println!("Waiting child thread...");
    handle.await.unwrap();
    println!("Child thread is ended.");

    // let token_name: String = token_contract
    //     .query("name", (), None, Options::default(), None)
    //     .await
    //     .unwrap();

    // let total_supply: U256 = token_contract
    //     .query("totalSupply", (), None, Options::default(), None)
    //     .await
    //     .unwrap();

    // println!("Token name: {}, total supply: {}", token_name, total_supply);

    Ok(())
}
