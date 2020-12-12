## Wallet Service
### Setup
On the QLDB page on AWS management console, perform the following operation:
1. Create a ledger with name **bank-account-ledger** or any other name. Ensure to use the correct ledger name in the config file (Config.toml)
2. Create table
```
CREATE TABLE bank_accounts
```
3. Create index on table
```
CREATE INDEX ON bank_accounts (account_number)
```

### Run
In the project root directory, type the command below to run </br>
```
cargo run
```
Change the *http_port* and *ledger_name* in the configuration file (Config.toml) as you see fit.
Default Base URL: http://locathost:8080

### Rest Endpoints
1. `GET /account` - get all accounts
2. `GET /account/{account_number}` - get account details by **account_number**
3. `POST /account` - Create new account. This returns a JSON response including the account_number and default balance of 0.
4. `DELETE /account/{account_number}` - delete account by **account_number**
5. `POST /transaction` - Process transaction based on JSON payload.


### New account payload (/account)
```json
{
	"name": "Sam James",
	"phone": "2347038657970"
}
```

### Debit Payload for Transaction endpoint (/transaction)
```json
{
	"amount": "50",
	"recipient_account_number": "565656565",
	"transaction_type": "DEBIT"
}
```

### Credit Payload for Transaction endpoint (/transaction)
```json
{
	"amount": "100",
	"recipient_account_number": "565656565",
	"transaction_type": "CREDIT"
}
```

### Transfer Payload for Transaction endpoint (/transaction)
```json
{
	"amount": "50",
	"recipient_account_number": "565656565",
	"sender_account_number": "3971240165",
	"transaction_type": "TRANSFER"
}
```