# ChainSubscription Hub

## Project Title
ChainSubscription Hub

## Project Description
ChainSubscription Hub is a decentralized smart contract platform to manage subscription plans and user subscriptions with automated renewals. Built using Soroban on the Stellar blockchain, it provides transparent management, enforcing subscription rules, renewal cycles, and pausing or canceling subscriptions securely and trustlessly. It ensures that businesses and dApps can verify active user access rights entirely on-chain without relying on traditional centralized authentication.

## Project Vision
The vision of ChainSubscription Hub is to offer subscription-based businesses a decentralized, secure, and automated way to handle user subscriptions and renewals without relying on centralized intermediaries. This guarantees transparent access management and payment enforcement, increasing user trust, lowering operational platform fees, and maximizing overall business reliability.

## Key Features
- **Plan Management:** Admins create and manage subscription plans specifying unique IDs, names (Symbols), pricing, and custom durations.
- **User Subscriptions:** Users can subscribe to active plans securely with structural parameters to enable or disable auto-renewals.
- **Automated Renewal:** Subscriptions auto-renew based on duration and payment ledger tracking (payment logic external).
- **Subscription Cancellation:** Users can cancel subscriptions at any time, disabling immediate or future automatic renewal states.
- **Immutable Records:** All plans and subscription states are recorded completely on-chain using Soroban persistent and instance storage for absolute auditability.
- **Access Control:** Admin-restricted plan management verified via cryptographic keys and user-restricted personal subscription control.
- **Transparent Status:** Publicly accessible subscription status functions (`check_status`) allow external backends to verify valid access instantaneously.

## Usage Instructions
1. **Set Admin:** Deploy the contract and invoke the `initialize` function to assign the dedicated admin for subscription control.
2. **Create Plans:** Admin adds custom subscription plans by defining parameters for name, price, and duration (in seconds).
3. **Subscribe:** Users subscribe to target plans by specifying a `plan_id` and optionally enabling the `auto_renew` flag.
4. **Auto Renew:** Auto renew can be triggered on cycle deadlines based on blockchain ledger timestamps.
5. **Cancel:** Users cancel subscriptions whenever desired to safely prevent future automatic cycles.
6. **Query:** Anyone can query individual subscription info or active plan details transparently via read-only endpoints.

## Future Scope
- **Payment Integration:** Integrate Stellar Asset Contract (SAC) or custom Soroban tokens to fully automate native balance transfers upon subscription updates.
- **Multi-tier Subscriptions:** Support multi-level upgrades, downgrades, or bundled cross-contract service plans.
- **Trial Periods:** Add programmatic support for trial durations, promotional coupons, and discount codes.
- **User Dashboards:** Build a clean React/Next.js frontend user interface integrated with Freighter Wallet for seamless visual account management.
- **Notification System:** Implement off-chain event listeners to send alerts for renewals, approaching expiration deadlines, or renewal failures.
- **Cross-Platform Sync:** Sync decentralized identity subscription logs across multi-chain services safely.
- **Compliance Tools:** Automate on-chain tax calculations and accounting regulatory compliance reporting.

## Technology Stack
- Rust and Soroban SDK for secure, deterministic, and gas-efficient smart contract development.
- Stellar blockchain (Testnet) for decentralized, fast, and immutable ledger state management.
- Cryptographic signing (`require_auth`) and ledger timestamp records for secure subscription enforcement.

## Contribution
Community contributions are welcomed from blockchain developers and subscription platform experts. Feel free to fork the repository and submit pull requests to assist in further development.

## License
This project is licensed under the MIT License.

## Contract Detail

ID: CDOLFWOPMMIPMDM6EBMBKVUUO22OAA5KM5UGJYQMNOUZ32VNWZBII7BG
![alt text](https://ibb.co/4n8ndw5R)
