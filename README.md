# AOS Auth: Authentication for Actix on Sled

## What AOS Auth?

Actix is an amazingly fast http framework written in rust.
Sled is a fast embedded key value store, also written in rust.

Together they make an application that is

- Deployable as a small, single binary for server and database,
- Very fast, and
- Flexible.

However, the ecosystem is still very young.  
As a result of that, I spent a fair amount of time figuring out how to do something as basic as authentication in this system.
I created this repository to save me some time when I create the next server around these technologies and am sharing it with the hope of saving you time as well.

## Why?

It's an odd time in the history of the web because computers are becoming more and more powerful, and at the same time it's becoming more and more common to build distributed apps with perhaps tens or hundreds or thousands of virtual machines working in tandem.

But it's also interesting to consider the other extreme.  If you use a single computer extremely efficiently, how far can you go and how reliable can you get?  At the very least, in doing this you can achieve a speed that just isn't possible in a distributed system, at least without compromising on consistency.  You can also remove a lot of complexity, which has its own maintenance cost, and reliability issues.  Actix on sled is one approach to exploring that second extreme.

## Notes on authentication

This app provides an example of using an opaque session token authentication flow with actix on sled.

- The token is saved in sled and points to a user_id.
- Password hashing is handled by Argonautica, as used in `src/hashing.rs`

## Notes on architecture

- All interactions with the sled database are handled by a DataLayer object in `src/datalayer.rs`, with the goal that the rest of the app doesn't need to know that sled exists -- instead it calls methods on DataLayer.
- Data chunks like email addresses and ids and hashed passwords are wrapped in types that are mostly defined in `atomics.rs`.  These types make it easy to define validation and serialization for each type.
- In the sled database, data is stored in json strings.  While json isn't the fastest or most compact way to store data in sled, I find that serialization isn't slow and that having the raw data be human readable has some value.
- Instead of handling authentication in middleware, I use Actix's extractor pattern.  In this app that means that when the `index` in `src/app/views` wants to know who the user is, it adds the `AuthedUser` type to the function signature, as in `index(user: AuthedUser)`.  Behind the scenes, this triggers `AuthedUser::from_request` as defined in `auth/extractors/authed_user.rs`.
- In terms of data layout in sled, I'm staying very granular.  That is, rather than store a serialized user object, I have separate trees for separate fields such as `user_id__email` or `email__password`, where the naming scheme is `key_name__value_name`.  This granular storage will end up being more expensive in terms of query count if many fields are needed.  But since my apps are currently in early design phase I'm using this structure because it makes adding new fields trivial and also because, by making it natural to request exactly the data that you need, I hope that it will lead to code that is more aware of exactly what data is passed where and so more efficient.
- I take the opposite approach when dealing with app configuration -- instead of having many config bits in different places, I'm trying to deal with all app configuration as defined in the `Config` struct in `src/lib.rs`.

## Performance

- On my laptop,
  - initial password hashes using argonautica take ~1 second, while subsequent verifications take about 1/3 s.  These are deliberately slow to make certain password attacks harder.
  - An unauthenticated view with simple template rendering using Askama takes 35-300 microseconds as reported in the logs.
  - The authenticated index view, which also uses an Askama template but this time with a user id, takes  ~35 to 300 microseconds as reported in the logs.

I don't know what the variability comes from, and both seem to mostly be 35-70 microseconds but with occasional slow requests that take as much as 200 or 300 microseconds.  At any rate, it's fairly fast but could certainly be faster.  Also, the time associated with a single call to sled isn't easy to resolve with that much variability.

## Usage

### Install rust

If you haven't already, [Install Rust](https://www.rust-lang.org/tools/install).

### Clone this repository

```bash
git clone git@github.com:dhbradshaw/aos_auth.git
```

### Enter repository directly

```bash
cd aos_auth
```

### Create a new user

```bash
cargo run --bin newuser test@test.come testpassword
```

[The first time you run this, it will download and compile all the dependencies.  You'll be waiting a little bit!]

### Start server

```bash
cargo run
```

### Open web browser

Open your web browser to <http://127.0.0.1:7654>

### Enter email and password

If you want login to succeed, use an email and password that you entered using the newuser command above.

### Take a look at your terminal

You should see logging associated with each page request.  Times may be on the order of a millisecond.

### Build release binary

Times for the debugging version of the binary are slow on my laptop -- on the order of almost a millisecond per request.  If you want them to be 10 times as fast, create a slower but more optimized release build and try that.

```bash
cargo run --release
```
