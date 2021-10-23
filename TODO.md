# TODO

## Contract

[x] - Define base contract entrypoint
[x] - Ensure Xargo config + Cargo features work together
[x] - Write sanity test
[x] - Write deploy scripts
[x] - Create and fund devnet solana-wallet
[x] - Build + deploy contract
[x] - Grok a test client scripts
[x] - Tweak for our contract
[x] - Ensure it works with the deployed contract

### Port solidity members

#### Data

[ ] - map of whitelisted address
[ ] - map of tokens purchased by an address
[ ] - Datum for PresaleToken
[ ] - Datum for RaisedToken
[ ] - min/max token allocs
[ ] - hardcap on raised tokens
[ ] - token per usd
[ ] - total raised in usd
[ ] - participating addresses
[ ] - total percentage distributed
[ ] - isActive
[ ] - isWhitelist (once off, can't be turned back on)

#### Methods

[ ] - function addToWhitelist(address[] memory accounts) public onlyOwner
[ ] - function startPresale() public onlyOwner
[ ] - function stopPresale() public onlyOwner
[ ] - function stopWhiteList() public onlyOwner
[ ] - function isWhitelisted(address account) external view returns (bool)
[ ] - function buy(uint amount) public
[ ] - function distributeTokens(uint percentageOfAmountOwed) public onlyOwner
[ ] - function withdrawFunds () external onlyOwner
[ ] - function withdrawUnsoldTokens() external onlyOwner
[ ] - Test against completed contract on the devnet
[ ] - Write production build scripts
[ ] - Ensure production key sigs and addresses are valid and secure  
[ ] - Deploy and test production contract

## Frontend

[ ] - Build base view
[ ] - Integrate contract logic with web3
[ ] - Perform unit tests
[ ] - Polish the view
