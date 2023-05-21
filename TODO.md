# Todo

# V1
- Balance management
  - Add balance (compute initial value based on date and transactions)
  - Remove balance
  - Button to update balance to match delta
- Transaction management
  - Manually add transaction
  - Edit transactions
  - Remove transactions
  - Add transaction to fix balance diff in balance table
  - In balance manager: button to insert new transaction equal to diff
- Import transactions from statements
  - Conversion from parsed statement to Statement and list of Transactions
  - Preview (and edit? filter?) transactions before importing
  - Prevent/warn about reimporting same statement
- Flow management
  - Filter list of transactions to just unassigned transactions
  - Simple case: create flows for transactions by just assigning category
  - Transaction group editor: add multiple transactions to a single group, remove transactions, add additional flows, edit flows
  - Flow view
- Automtic flow creation
  - Set up regex-based rules for transactions, automatically apply to new imported transactions to create flows
- Data visualization
  - Spending by category by time period
  - Net worth over time
