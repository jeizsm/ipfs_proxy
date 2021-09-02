# How to run
install and run ipfs daemon [ipfs instructions](https://docs.ipfs.io/install/command-line/#official-distributions)
install and run postgresql 
edit environment variables if necessary in env file
cargo run
# How to test
`curl --header 'Content-Type: application/json' -v -X POST http://localhost:5002/service_api/sign_up -d'{"username": "hello", "password": "world"}'`

`curl --header 'Content-Type: application/json' -v -X POST http://localhost:5002/service_api/login -d'{"username": "hello", "password": "world"}'`

Get jwt token from response and set it JWT_TOKEN variable

`curl --header "Authorization: Bearer $JWT_TOKEN" --header 'Content-Type: application/json' -v -X POST http://localhost:5002/service_api/api_key`

Get api token from response and set it API_KEY variable

`curl --header "Authorization: Bearer $API_KEY" -v -X POST http://localhost:5002/api/v0/swarm/peers`

`curl --header "Authorization: Bearer $JWT_TOKEN" --header 'Content-Type: application/json' -v -X DELETE http://localhost:5002/service_api/api_key -d"{\"api_key\": \"$API_KEY\"}"`

`curl --header "Authorization: Bearer $JWT_TOKEN" --header 'Content-Type: application/json' -v -X GET http://localhost:5002/service_api/api_key`
