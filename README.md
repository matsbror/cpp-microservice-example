# Microservice example
A very simple microservice example with two (not yet) dockerized services

Start service 1:

```
cd service1
cargo run
```

In another terminal, start service 2:
```
cd service2
cargo run
```

In yet another terminal, access the service:

The add and sub requests will run only on service 1:

```
curl --location 'http://127.0.0.1:8001/math/add' --header 'Content-Type: application/json' --data '{  "arg1": 45, "arg2": 12 }'
```

```
curl --location 'http://127.0.0.1:8001/math/add' --header 'Content-Type: application/json' --data '{  "arg1": 45, "arg2": 12 }'
```
The mul and div requests will first go to service 1 and then to service 2:

```
curl --location 'http://127.0.0.1:8001/math/mul' --header 'Content-Type: application/json' --data '{  "arg1": 45, "arg2": 12 }'
```

```
curl --location 'http://127.0.0.1:8001/math/div' --header 'Content-Type: application/json' --data '{  "arg1": 45, "arg2": 12 }'
```

