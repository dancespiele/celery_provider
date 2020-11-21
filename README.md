# Celery consumer

This is an example of provider with [Rust Celery](https://rusty-celery.github.io/) and [Warp](https://github.com/seanmonstar/warp) which will get the complete result task from [Sled](https://sled.rs/) database.

# how it works

1. Install [Rabbitmq](https://www.rabbitmq.com/)

2. Clone the project: `git clone https://github.com/dancespiele/celery_provider.git` 

3. Inside of the project directory add the .env file with the path of you Sled database (notice that it will be sharing with the consumer):

```
SLED_URL=[YOUR SLED PATH]
AMPQ_ADDR=[YOUR AMPQ URL]
```

4. Execute `cargo run`

5. Run the [Consumer](https://github.com/dancespiele/celery_provider) example to see all the complete flow

6. Execute this curl

```
curl --location --request GET 'http://localhost:3000/hello' \
--header 'Content-Type: application/json'
```

## Do you like it?

If you liked, please help me to make more different examples in rust with:

- BAT rewards in case that you use [Brave Browser](https://brave.com/)
- [Github Sponsors](https://github.com/sponsors/dancespiele)
- Burst coins to the address BURST-DPN6-2AT3-FCRL-9BBKG

## License

Celery consumer is [Apache-2.0](LICENSE.md) licensed