# SimpleLogin Rs

This crate is a wrapper for the [simplelogin.io](https://simplelogin.io/) api.

<br>

## Details

- Create the client

``` rust
let mut client = SimpleLoginClient::new("app.simplelogin.io");

client.token = Some("TOKEN");
```

- Make requests

```rust
// ** User Infos **
let response = client.account().get_user_info().await;

// ** Get Alias Options **
let response = client.alias().options().await;

// ** Create Mailbox **
let response = client.mailbox().create("mailbox_to_add").await;

// ** List Custom Domains **
let response = client.custom_domain().list().await;

// ** Get Settings **
let response = client.setting().get().await;
```

<br>

#### License

<sup>
Licensed under <a href="LICENSE">GPL-3.0
</sup>
