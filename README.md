# iUSD

  <div align="center"">
    <img src="https://github.com/1010adigupta/ink-Stable-Coin/assets/121158631/773705e3-96c5-485e-b2e9-5634795940c1" alt="iusd">
  </div>
  


## Description

iUSD is a crypto-collateralized stable coin implementation written in rust (ink! framework) for ALEPH ZERO, Polkadot and Substrate-based parachains.

## Architecture

![architecture](https://github.com/1010adigupta/ink-Stable-Coin/assets/121158631/2d66d4b3-4f86-43aa-8177-af10ccbb445d)

The iUSD operates with the following key components and responsibilities:

### User Actions

1. **deposit_and_mint Function**: Users can initiate the deposit_and_mint function. This function involves the following steps:
   - Calls the price feed function of the DIA oracle contract on the chain to retrieve the price of the collateral token in USD, ensuring the stable coin is pegged to USD.
   - Deposits collateral into the system.
   - Mints a corresponding amount of stable coin, providing the user with a stable coin balance, in accordance with over-collaterisation percentage.

2. **redeem_collateral Function**: Users have the option to redeem their collateral by calling the redeem_collateral function. This process includes:
   - Liquidating a portion of their collateral.
   - Burning the necessary amount of stable coin.
   - Updating the user's health factor.

### Health Factor Management

3. **Health Factor Calculation**: The system continuously calculates a user's health factor, which is determined by the ratio of the total user_collateral amount to the user_stable_coin balanc and LIQUIDATION_THRESHOLDe. A predefined LIQUIDATION_THRESHOLD = 2, is maintained to ensure a 200% over-collateralization.

4. **Liquidation Function**: In cases where a user's health factor falls below the threshold of 1, any participant can initiate the liquidate function. The liquidation process involves:
   - Liquidating the user's collateral.
   - Burning the necessary amount of stable coin to improve the user's health factor.
   - Updating the health factor of the affected user.

5. **Liquidator Bonus**: As an incentive for maintaining system health, the participant who initiates the liquidation process is rewarded with a bonus. This bonus is typically 10% of the collateral amount that was successfully liquidated.

6. **Collateral Redemption**: In scenarios where users wish to redeem their collateral, the process includes:
   - Redeeming the collateral held in the system.
   - Burning the required amount of stable coin.
   - Updating the user's health factor.

This architecture ensures the stable coin's value is maintained by tracking collateral values, protecting users from under-collateralization, and providing incentives for maintaining system health. Users can confidently interact with the stable coin system, knowing that it is designed to operate in a secure and sustainable manner.

## SetUp

Make sure you have installed all of the following prerequisites on your development machine:
- [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [cargo-contract](https://github.com/paritytech/cargo-contract#installation), a CLI tool for helping setting up and managing WebAssembly smart contracts written with ink!

## Deployment

To deploy this contract, follow these steps:

1. Clone the project:

   ```bash
   git clone https://github.com/1010adigupta/ink-Stable-Coin.git
   ```

2. Navigate to the project directory:

   ```bash
   cd ink-Stable-Coin
   cd contracts
   cd stable-coin
   ```

3. Generate the contract binaries:

   ```bash
   cargo contract build
   ```

4. A stable-coin-contract.contract binary will be generated. Now, go to [substrate-contracts-ui](https://contracts-ui.substrate.io/), select ALEPH ZERO testnet, and upload the .contract binary.

5. Instantiate the contract by supplying the dia-oracle address on ALEPH ZERO testnet as an argument to the constructor. The dia-oracle contract address is:

   ```
   5F5z8pZoLgkGapEksFWc2h7ZxH2vdh1A9agnhXvfdCeAfS9b
   ```

## Roadmap

Our roadmap includes the following milestones:

- **Savings Feature**: Holders of the iUSD stable coin can earn the iUSD savings rate (iUSR) on the iUSD they hold by locking it into a special smart contract. There are no fees involved, no geographic constraints, and no liquidity impediments—no minimum deposit is required to earn the iUSR, and all or any portion of Dai can be withdrawn at any time.

- The iUSR is not only a propeller of financial freedom, offering complete user control, but also game-changing for the DeFi movement.

- **AZERO Staking**: Since our stable coin is over collaterized, we can divert a particular amount of AZERO collateral in our contract to the AZERO staking feature on ALEPH ZERO chain, this will enable efficient usage of stored collateral and generate revenue from the same.

- **Add More Integrations**: We are actively working on integrating our stable coin with various DeFi projects and platforms to enhance its usability.
- **Add ink! e-2-e tests**: Next step is to add ink! e-2-e tests, currenltly all funcitons have been tested by deploying contract on substrate-contracts-ui and testing every functions there.
