# Point-Api
## Running
From source:
Install Rust and run:
```
cargo run -- ./point.sqlite
```

Otherwise, run the latest release binary
The endpoint will be accessible at http://localhost:4000

## Endpoints
Endpoints were designed to match the data in the email as best as possible.

### POST /orders/new
This takes a new order with an associated customer and optional reward parameters as json parameters
```
curl -X POST http://localhost:4000/orders/new
   -H "Content-Type: application/json"
   -d '{"order": {"id": "104fd7e0-a188-4ffd-9af7-20d7876f70ab", "paid": 10000, "currency": "jpy"}, "customer": {"email": "exampple@lunaris.jp", "phone": null}'
```
The following object can optionally be added to the json data above to modify the reward rate for the given upload.
```
"rewardParams": {"amount": .02}
```

This currently adds customers if they don't already exist, some discussion should be held as to how this should actually be handled.
Without batching on this route if an order is rejected for some reason, but the user changes go through the user would still have access to their points.

### GET /user/{user}/balance
This gets the balance of {user} in the url
```
https://localhost:4000/user/example@lunaris.jp/balance
```
This route should have authentication

### POST /user/{user}/add
Adds balance to {user} in path with the amount given in the posted json data.
```
curl -X POST http://localhost:4000/user/example@lunaris.jp/add -H "Content-Type: application/json" -d '{"amount": 2}'
```
This route should have authentication behind it
### POST /user/{user}/sub
Same as add, but subtracts

## Performance
Performance metrics were gathered using wrk to spawn concurrent connections and gather data regarding latency and requests/second on development machine.
The server was hosted on an AWS ec2 t2.micro instance, with a ping of about 15ms from the consuming machine.
The get balance endpoint was selected 
I'd like to gather metrics on the Post routes in the future as well 


### 100 concurrent connections
| Thread Stats       | Avg     | Stdev  | Max     | +/- Stdev |
|--------------------|---------|--------|---------|-----------|
| Latency            | 12.86ms | 4.41ms | 247.75ms| 95.59%    |
| Req/Sec            | 629.56 | 56.11   | 750.00  | 73.92%    |

| Total Stats        |         |
|--------------------|---------|
| Requests/Sec       | 7516.09 |
| Transfer/Sec       | 1.28MB  |

### 400 concurrent connections
| Thread Stats       | Avg     | Stdev  | Max     | +/- Stdev |
|--------------------|---------|--------|---------|-----------|
| Latency            | 30.38ms|4.63ms | 198.84ms|   97.11%|
| Req/Sec            | 659.88|  401.02| 1.21k|   54.44%|

| Total Stats        |         |
|--------------------|---------|
| Requests/Sec       |  7842.91|
| Transfer/Sec       | 1.33MB  |

### 1000 concurrent connections
| Thread Stats       | Avg     | Stdev  | Max     | +/- Stdev |
|--------------------|---------|--------|---------|-----------|
| Latency            | 29.93ms | 4.60ms| 327.81ms |  95.36%   |
| Req/Sec            | 786.19  | 606.29 |2.10k    |49.43%     |

| Total Stats        |         |
|--------------------|---------|
| Requests/Sec       |7786.11  |
| Transfer/Sec       |1.32MB   |

### 2000 concurrent connections
| Thread Stats       | Avg     | Stdev  | Max     | +/- Stdev |
|--------------------|---------|--------|---------|-----------|
| Latency            |30.33ms  |4.79ms  |326.26ms |96.52%     |
| Req/Sec            |0.99k    |765.34  |2.38k    |52.32      |

| Total Stats        |         |
|--------------------|---------|
| Requests/Sec       |7812.66  |
| Transfer/Sec       |1.33MB   |

## TODO
- Add authentication to set/checkbalance routes and rewardparameters.
- Add proper batching to sql statements- Add proper testing to check routes are robust
- Sqlite was used for quick development and making it easy to setup testing. Databases are parameterized in such a way that it is simple to change to postgres
- A discussion should be had as to how multiple table additions should be managed. I generally try to structure endpoints so that a single POST leads to a single table being added when possible. But in this case I added multiple endpoints for simplicity's sake.
- I'd prefer not to use email as the cannonical user id, but it looks like that's the structure for the data given, so I left it as is.
- Depending on requirements of business, it might make sense for add and sub to be either separate or together.
