# Money App
The Money App is a App for budgeting, analysing and visualization of wealth, income and spending.
It is based around transaction and stores all it's information in form of generic transactions, which can be imported from diffenrent banks. 
The places where wealth can be stored is called a pod.
A transaction is always one of three different types (IN, OUT, MOVE) depending wheater the receiver, the sender or both are pods of the user.
Each transaction can have multple budgets, inbudgets and debts and arbitrary other tags.

The App is implemented through a Golang-Backend and a Svelte-Kit-Frontend.
For them to work together some sort of authentication proxy (like [proxauth](https://github.com/xilefmusics/proxauth)) is needed since the backend doesn't handle user authentication itself.

## Backend
The backend at the moment is readonly, but the data module is prepared to also accept multiple write processes.
It selects the data it operates on through a header `user: <username>`.
The currently supported requests are

|Endpoint|Method|Description|
|--------|------|-----------|
|/transactions|GET|Returns all transactions.|
|/lint|GET|Returns all linting problems of transactions.|
|/reindex|GET|Sorts transactions by date and overwrites the ID with the index.|
|/pods|GET|Returns all pods.|
|/debts|GET|Returns all debts.|
|/budgets|GET|Returns all budgets.|
|/inbudgets|GET|Returns all inbudgets.|
|/tags|GET|Returns all tags.|
|/history/debt|GET|Returns the history of debts.|
|/history/budgets|GET|Returns the history of budgets.|
|/history/inbudgets|GET|Returns the history of inbudgets.|
|/history/pod|GET|Returns the history of pods.|
|/history/wealth|GET|Returns the history of wealth.|