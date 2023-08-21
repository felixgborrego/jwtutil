# JWT utils

Small CLI tool to debug JWT tokens implemented in Rust.

## Usage

```sh
# Display a token
jwtutil <jwtToken>

# Or display a token from a file
jwtutil <file>
```

example output:

```json
{
  "aud": [
    "https://aaa.bbb.com/api/v2/",
    "https://test.eu.auth0.com/userinfo"
  ],
  "azp": "xxxxxxxx",
  "exp": 1692120799,
  "iat": 1692112199,
  "iss":  "https://test.eu.auth0.com/",
  "permissions": [],
  "scope": "openid",
  "sub": "google-oauth2|aaaa"
}
 * exp[expiration] time: 31 minutes ago
 * iat time: 2 hours ago
```
### Install
```sh   
# Locally
cargo install --path .  
# from github
cargo install --git https://github.com/felixgborrego/jwtutil
``````