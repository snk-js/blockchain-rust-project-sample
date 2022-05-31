### Coding Challenge Guidelines

- Implement post upvote
  - This should take in a Post ID and apply a single upvote to the Post.
  - What if the same user upvotes twice?
- Implement post list
  - This should take in a PostQuery object and return an array of Posts relative to the query.
  - How would you validate the input?
  - Bonus: Implement a `Hot` sort using [this algorithm](https://medium.com/hacking-and-gonzo/how-reddit-ranking-algorithms-work-ef111e33d0d9) or make your own.
- Implement posts by user
  - This should take in a PostQuery and a username and return an array of Posts that are by that user.

### Evaluation Criteria

- Rust best practices
- Completeness: did you complete the features?
- Correctness: does the functionality act in sensible, thought-out ways?
- Maintainability: is it written in a clean, maintainable way?
- Scalable: What if there was 100,000 Posts? 1,000,000 Posts?

### Useful Links

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

## Running the project locally

Install Internet Computer Client

```bash
DFX_VERSION=0.9.3 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

You may need the Rust WASM target added

```bash
rustup update
rustup target add wasm32-unknown-unknown
```

To run the project you will need to do the following.

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

# Inserts some data into the canister
./scripts/setup.sh
```

Once the job completes, your application will be available at `http://127.0.0.1:8000?canisterId={__Candid_UI}&{dscvr_takehome_rust}`. Which is found in `.dfx/local/canisters/canister_ids.json`

## Issue handling

### Reinstalling rust project

Data model changes usually require a reinstall. There are work arounds for this, but for this take home test, reinstalling is fine.

```bash
npm run reinstall
```

Type `yes` at WARNING

### CodeSubmit

Please organize, design, test, and document your code as if it were
going into production - then push your changes to the master branch.

Have fun coding! 🚀
