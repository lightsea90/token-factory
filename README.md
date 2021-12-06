Token Factory
==================

This is a smart contract running on NEAR Protocol. It could be used to issue a fungible token with a simple tokenomic.


# Token factory demo
```
$ near create-account treasury.tokenhub.testnet --masterAccount tokenhub.testnet --initialBalance 2
$ near call token-factory.tokenhub.testnet --accountId harrynguyen006.testnet new

# register
export current=$(date +%s)
$ near call token-factory.tokenhub.testnet --accountId harrynguyen005.testnet register '{
    "ft_contract": "test003.token-factory.tokenhub.testnet", 
    "deployer_contract": "test003-deployer.token-factory.tokenhub.testnet",
    "total_supply": "100000000000000000",
    "token_name": "test003 name",
    "symbol": "test003",
    "decimals": 8,
    "initial_release": "20000000000000000",
    "vesting_start_time": "'$((current+5*60))000000000'",
    "vesting_end_time": "'$((current+15*60))000000000'",
    "vesting_interval": "'$((5*60))000000000'",
    "treasury_allocation": "10000000000000000"
}' --deposit 8

# deploy ft contract
$ near call token-factory.tokenhub.testnet --accountId harrynguyen005.testnet create_ft_contract '{
    "ft_contract": "test003.token-factory.tokenhub.testnet"
}' --gas 60000000000000

# deploy ft deployer contract
$ near call token-factory.tokenhub.testnet --accountId harrynguyen005.testnet create_deployer_contract '{
    "ft_contract": "test003.token-factory.tokenhub.testnet"
}' --gas 60000000000000

# issue token
$ near call token-factory.tokenhub.testnet --accountId harrynguyen005.testnet issue_ft '{
    "ft_contract": "test003.token-factory.tokenhub.testnet"
}' --gas 60000000000000

# setup token allocation
$ near call token-factory.tokenhub.testnet --accountId harrynguyen005.testnet init_token_allocation '{
    "ft_contract": "test003.token-factory.tokenhub.testnet"
}' --gas 60000000000000

# check token state
$ near view token-factory.tokenhub.testnet get_token_state '{
    "ft_contract": "test003.token-factory.tokenhub.testnet"
}'

# storage deposit
near call test003.token-factory.tokenhub.testnet storage_deposit '' --accountId harrynguyen005.testnet --amount 0.00125

# claim
$ near call test003-deployer.token-factory.tokenhub.testnet claim --accountId harrynguyen005.testnet --gas 60000000000000
$ near call test003-deployer.token-factory.tokenhub.testnet claim --accountId treasury.tokenhub.testnet --gas 60000000000000
```

Demo for new functions

```
$ near view token-factory.tokenhub.testnet list_all_tokens
View call: token-factory.tokenhub.testnet.list_all_tokens()
[
  'test001.token-factory.tokenhub.testnet',
  'test002.token-factory.tokenhub.testnet',
  'test003.token-factory.tokenhub.testnet',
  'TESTT.token-factory.tokenhub.testnet',
  'TESTT1.token-factory.tokenhub.testnet',
  'TESTT3.token-factory.tokenhub.testnet',
  'testt.token-factory.tokenhub.testnet',
  'testt1.token-factory.tokenhub.testnet',
  'mytot.token-factory.tokenhub.testnet',
  'ttss2.token-factory.tokenhub.testnet',
  'test004.token-factory.tokenhub.testnet',
  'tt2.token-factory.tokenhub.testnet',
  'tst2.token-factory.tokenhub.testnet',
  'tst3.token-factory.tokenhub.testnet',
  'ttst2.token-factory.tokenhub.testnet',
  'testwifi.token-factory.tokenhub.testnet',
  'lost.token-factory.tokenhub.testnet'
]

$ near view token-factory.tokenhub.testnet list_my_tokens '{"account_id": "harrynguyen.testnet"}'
View call: token-factory.tokenhub.testnet.list_my_tokens({"account_id": "harrynguyen.testnet"})
[ 'test005_harryng.token-factory.tokenhub.testnet' ]

$ near view token-factory.tokenhub.testnet get_token_state '{"ft_contract": "test005_harryng.token-factory.tokenhub.testnet"}'
View call: token-factory.tokenhub.testnet.get_token_state({"ft_contract": "test005_harryng.token-factory.tokenhub.testnet"})
{
  ft_contract: 'test005_harryng.token-factory.tokenhub.testnet',
  total_supply: '100000000000000000',
  token_name: 'test005 token',
  symbol: 'TEST005_HARRYNG',
  decimals: 8,
  ft_deployer: 'test005_harryng-deployer.token-factory.tokenhub.testnet',
  creator: 'harrynguyen.testnet',
  initial_release: '15000000000000000',
  vesting_start_time: '1638801669000000000',
  vesting_end_time: '1641393669000000000',
  vesting_interval: '86400000000000',
  treasury_allocation: '8000000000000000',
  ft_contract_deployed: 1,
  deployer_contract_deployed: 1,
  ft_issued: 1,
  allocation_initialized: 1
}

$ near view test005_harryng-deployer.token-factory.tokenhub.testnet check_account '{"account_id": "harrynguyen.testnet"}'
View call: test005_harryng-deployer.token-factory.tokenhub.testnet.check_account({"account_id": "harrynguyen.testnet"})
{
  allocated_num: '92000000000000000',
  initial_release: '15000000000000000',
  vesting_start_time: '1638801669000000000',
  vesting_end_time: '1641393669000000000',
  vesting_interval: '86400000000000',
  claimed: '0',
  claimable_amount: '15000000000000000'
}
```
