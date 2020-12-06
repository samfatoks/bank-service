## Rust Bank Service
### Assumptions
* Production calls to QLDB should be a lot faster. Therefore, I'm making multiple calls per each transactional operation to make it closer to a real world app.
* I’m using a 10 digit account number to represent the unique identifier that is known to the user(Buyers/Sellers). This is modeled after the 10 account number assigned by Nigerian banks.
* Amount is stored on ledger in the major currency form (not minor or cents).

### Good Practices
* Central error management for the application - This is made possible by the capabilities of the Rust language.
* Toml based configuration file for easy setup.
* Handling transaction requests (Credit, Debit, Transfer) with one endpoint but with varying payload field/values.
* The debit, credit and transfer operations are done within the transaction closure to allow full rollback in case any steps fail. Thanks for your amazing Rust QLDB Driver.

### Missing features
The following features are a most have for production applications but are missing in the app due to time constraint.
* Transaction Logging: Transactions are not currently logged to a transaction table on QLDB.
* Test - Lacking of unit and integration tests

### Core Rust libraries used
* QLDB Driver - https://github.com/Couragium/qldb-rs
* Ion Library - https://github.com/Couragium/ion-binary-rs
* Actix

### Issues
* I had problems using using the QLDB ledger created by David so I created a new ledger on my AWS account for the sake of this development.
* Calls to QLDB is a bit slow. I can attribute the latency partly to the location of the AWS region where I created the QLDB ledger (us-east-1)

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

### **Debit** Payload for Transaction endpoint (/transaction)
```json
{
	"amount": "50",
	"recipient_account_number": "565656565",
	"transaction_type": "DEBIT"
}
```

### **Credit** Payload for Transaction endpoint (/transaction)
```json
{
	"amount": "100",
	"recipient_account_number": "565656565",
	"transaction_type": "CREDIT"
}
```

### **Transfer** Payload for Transaction endpoint (/transaction)
```json
{
	"amount": "50",
	"recipient_account_number": "565656565",
	"sender_account_number": "3971240165",
	"transaction_type": "TRANSFER"
}
```