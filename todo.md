# todo

- [x] handle errors, when external api request fails
- [ ] change language enum from `Deutsch` to `German`, only serde serialize name should be `Deutsch`

- [ ] start whole app with docker-compose

## data

- [ ] do not always open and close sled db  
    possibly use postgresql db for higher concurrency
    or use [deadpool](https://github.com/bikeshedder/deadpool)

## fetching

- [x] select language for word
- [ ] select what external api to use
- [ ] seperate words with multiple meanings (`drone:1`)

## frontend

- [x] display all words
- [ ] create color scheme
- [ ] better styling
- [x] change the selected language to the one in session storage
