# Money App

The Money App is a App for budgeting, analysing and visualization of wealth, income and spending.
It is based around transaction and stores all it's information in form of generic transactions, which can be imported from diffenrent banks.
The places where wealth can be stored is called a pod.
A transaction is always one of three different types (IN, OUT, MOVE) depending wheater the receiver, the sender or both are pods of the user.
Each transaction can have multple budgets, inbudgets and debts and arbitrary other tags.

The App is implemented through a Golang-Backend and a Svelte-Kit-Frontend.
For them to work together some sort of authentication proxy (like [proxauth](https://github.com/xilefmusics/proxauth)) is needed since the backend doesn't handle user authentication itself.

## Backend

The backend stores the list of transactions per user.
It selects the data it operates on through a header `user: <username>`.
If a request changes data it adds a change event to a list of events.
The newest event can be rolledbacked through the endpoint `/undo`.

### GET /transactions

Returns a list of transactions.

| Filter   | Default |
| -------- | ------- |
| year     | \*      |
| month    | \*      |
| pod      | \*      |
| debt     | \*      |
| budget   | \*      |
| inbudget | \*      |
| type     | \*      |
| id       | \*      |

### POST /transactions

Receives a list of transactions as payload and creates new transactions out of it.
The given ID of the transactions gets ignored and a new one gets assigned.
In contrast to all the other endpoints it accepts the data in multiple different forms and can therefore be used to import transactions from multiple different sources.
The list of supported formats can be seen in the table below.

| Format         | Explanation                                 |
| -------------- | ------------------------------------------- |
| json           | The default format of this money app.       |
| csv (Barclays) | The csv export of the Barclays credit card. |
| csv (N26)      | The csv export of the N26 bank.             |

### PUT /transactions

Receives a list of transactions as payload and updates the transactions given the ID.
It returns all change transactions.
If no transactions with a given ID exists it gets ignored and is missing in the result.

### DELETE /transactions

Receives a list of transactions and deletes them given their ID.

### GET /pods

Returns a list of all the pod names that occure in the sender or receiver field of the transactions.

### GET /debts

Returns a list of all the debt names that occure in the transactions.

### GET /budgets

Returns a list of all the budget names that occure in the transactions.

### GET /inbudgets

Returns a list of all the inbudget names that occure in the transactions.

### GET /tags

Returns a list of all the inbudget names that occure in the transactions.

### GET /history/...

The history endpoint returns a list of datapoints of different kindes given a timespan.
The default format of a datapoint is the absolute value at a specific time and the difference since the last datapoint.

```json
{
  "value": "<value>",
  "diff": "<diff>"
}
```

The supported history data series are:

- budgets
- inbudgets
- pod
- wealth

The supported params to change the dataseries are:

| Param | Function                                     | Default |
| ----- | -------------------------------------------- | ------- |
| year  | The amount of years between the datapoints.  | 0       |
| month | The amount of months between the datapoints. | 0       |
| day   | The amount of days between the datapoints.   | 0       |
| len   | The amount of datapoints                     | 1       |

### GET /lint

Returns a list of anomalies inside the transactions.
This is not yet a full list, but some lint rules.
The current list of rules is:

| Rule | Description                                                                                    |
| ---- | ---------------------------------------------------------------------------------------------- |
| 1    | If the transaction type is `in` the budget sum has to be empty.                                |
| 2    | If the transaction type is `in` the inbudget sum plus the debt sum has to be equal the amount. |
| 3    | If the transaction type is `out` the inbudget sum has to be empty.                             |
| 4    | If the transaction type is `in` the budget sum plus the debt sum has to be equal the amount.   |

### GET /reindex

Sorts transactions by date and overwrites the ID with the index.
This schould only be used if you know, what your doing.

### GET /undo

Undoos the last action and returns the applied event. 
