Hello, 

We are very happy on the prospect of working with you. Before continuing forward with the hiring process, we would like to have a technical test in order to evaluate your programming skills. We are looking forward to seeing how you develop this test!   

## Instructions

For this test you will need to implement a Ledger. A ledger is the principal book or computer file for recording and totaling economic transactions measured in terms of a monetary unit. Think of it like the bank with the different bank accounts where you have funds transfers and balance.

For your understanding of the use case, this ledger will have 2 accounts groups: 

- Buyer: Which will mainly send money to sellers.
- Sellers: Which will mainly receive money from buyers, even concurrently.

Note: _These two groups don't necessarily need to be represented in the data model or code._

It is very important that any account in the Ledger cannot have, under any circumstance, a negative balance. Additionally, it needs to handle considerable concurrent traffic on the same account without failing. 

For this exercise you will only need to implement the transfer between accounts. The transfer can be between any account, not only buyers and sellers.

The Ledger will be implemented as an HTTP service and will use Amazon's QLDB. You will be provided an Amazon AWS credentials with access to a QLDB instance. 

We will look at the process and the final code, so provide git commits of the process during the development.
 
We will need to run the code in order to test it. Provide clear instructions for it.

You are free to choose, for the implementation, any programming language from this list: Typescript, Rust, Java, Go, Python, C++

And finally, explain the good practices used on this code and the reasons for the chosen internal design as well as any possible problem/concern that may arise because of the chosen design.

Good luck!
