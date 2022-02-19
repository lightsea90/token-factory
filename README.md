Token Factory
==================

This is a smart contract running on NEAR Protocol. It could be used to issue a fungible token with a simple tokenomic.


# Token factory demo
```
$ near create-account treasury.tokenhub.testnet --masterAccount tokenhub.testnet --initialBalance 2
$ near call tokenhub.testnet --accountId harrynguyen006.testnet new '{"owner_id": "harrynguyen.testnet"}'

# register
export current=$(date +%s) test_id=test001
$ near call tokenhub.testnet --accountId harrynguyen005.testnet register '{
    "ft_contract": "'${test_id}'.tokenhub.testnet", 
    "deployer_contract": "'${test_id}'-deployer.tokenhub.testnet",
    "total_supply": "100000000000000000",
    "token_name": "'${test_id}' name",
    "symbol": "'${test_id}'",
    "decimals": 8,
    "allocations": {
        "treasury.tokenhub.testnet": {
            "allocated_percent": 800,
            "initial_release": 0,
            "vesting_start_time": "'$((current+5*60))000000000'",
            "vesting_end_time": "'$((current+15*60))000000000'",
            "vesting_interval": "'$((5*60))000000000'"
        },
        "harrynguyen005.testnet": {
            "allocated_percent": 9200,
            "initial_release": 1500,
            "vesting_start_time": "'$((current+5*60))000000000'",
            "vesting_end_time": "'$((current+15*60))000000000'",
            "vesting_interval": "'$((5*60))000000000'"
        }
    }
}' --deposit 8

$ near view tokenhub.testnet list_token_states '{"token_contracts": ["test001.tokenhub.testnet"]}'
View call: tokenhub.testnet.list_token_states({"token_contracts": ["test001.tokenhub.testnet"]})
[
  {
    ft_contract: 'test001.tokenhub.testnet',
    ft_deployer: 'test001-deployer.tokenhub.testnet',
    creator: 'harrynguyen005.testnet',
    ft_contract_deployed: 0,
    deployer_contract_deployed: 0,
    ft_issued: 0,
    allocation_initialized: 0
  }
]
[2022-02-19T08:16:02+07:00] harryng@harryng-desktop:/stuffs/projects/token-factory [*main]
$ near view tokenhub.testnet list_all_token_contracts
View call: tokenhub.testnet.list_all_token_contracts()
[ 'test001.tokenhub.testnet' ]

# deploy ft contract
$ near call tokenhub.testnet --accountId harrynguyen005.testnet create_ft_contract '{
    "ft_contract": "'${test_id}'.tokenhub.testnet"
}' --gas 60000000000000

# deploy ft deployer contract
$ near call tokenhub.testnet --accountId harrynguyen005.testnet create_deployer_contract '{
    "ft_contract": "'${test_id}'.tokenhub.testnet"
}' --gas 60000000000000

# issue token
$ near call tokenhub.testnet --accountId harrynguyen005.testnet issue_ft '{
    "ft_contract": "'${test_id}'.tokenhub.testnet"
}' --gas 60000000000000

# setup token allocation
$ near call tokenhub.testnet --accountId harrynguyen005.testnet init_token_allocation '{
    "ft_contract": "'${test_id}'.tokenhub.testnet"
}' --gas 60000000000000

# check token state
$ near view tokenhub.testnet get_token_state '{
    "ft_contract": "'${test_id}'.tokenhub.testnet"
}'

# storage deposit
near call ${test_id}.tokenhub.testnet storage_deposit '' --accountId harrynguyen005.testnet --amount 0.00125

# claim
$ near call ${test_id}-deployer.tokenhub.testnet claim --accountId harrynguyen005.testnet --gas 60000000000000
$ near call ${test_id}-deployer.tokenhub.testnet claim --accountId treasury.tokenhub.testnet --gas 60000000000000
```

Demo for new functions

```
$ near view tokenhub.testnet list_all_tokens
View call: tokenhub.testnet.list_all_tokens()
[
  'test001.tokenhub.testnet',
  'test002.tokenhub.testnet',
  'test003.tokenhub.testnet',
  'TESTT.tokenhub.testnet',
  'TESTT1.tokenhub.testnet',
  'TESTT3.tokenhub.testnet',
  'testt.tokenhub.testnet',
  'testt1.tokenhub.testnet',
  'mytot.tokenhub.testnet',
  'ttss2.tokenhub.testnet',
  'test004.tokenhub.testnet',
  'tt2.tokenhub.testnet',
  'tst2.tokenhub.testnet',
  'tst3.tokenhub.testnet',
  'ttst2.tokenhub.testnet',
  'testwifi.tokenhub.testnet',
  'lost.tokenhub.testnet'
]

$ near view tokenhub.testnet list_my_tokens '{"account_id": "harrynguyen.testnet"}'
View call: tokenhub.testnet.list_my_tokens({"account_id": "harrynguyen.testnet"})
[ 'test005_harryng.tokenhub.testnet' ]

$ near view tokenhub.testnet get_token_state '{"ft_contract": "test005_harryng.tokenhub.testnet"}'
View call: tokenhub.testnet.get_token_state({"ft_contract": "test005_harryng.tokenhub.testnet"})
{
  ft_contract: 'test005_harryng.tokenhub.testnet',
  total_supply: '100000000000000000',
  token_name: 'test005 token',
  symbol: 'TEST005_HARRYNG',
  decimals: 8,
  ft_deployer: 'test005_harryng-deployer.tokenhub.testnet',
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

$ near view test005_harryng-deployer.tokenhub.testnet check_account '{"account_id": "harrynguyen.testnet"}'
View call: test005_harryng-deployer.tokenhub.testnet.check_account({"account_id": "harrynguyen.testnet"})
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
