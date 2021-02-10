# AOS Auth: Authentication for Actix on Sled

This repo attempts to show a reasonable pattern for building an app with an opaque session token authentication flow.

## Notes on authentication

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

I don't know what the variability comes from, and both seem to mostly be 35-70 microseconds but with occasional slow requests that take as much as 200 or 300 microseconds.  At any rate, it's fairly but could certainly be faster.  Also, the time associated with a single call to sled isn't easy to resolve with that much variability.
