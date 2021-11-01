
# Running the server

For the server to start, it must connect to a mongodb database. 
The server reads information about this database from environment
variables. These can be set manually, or put in a dotenv (.env) file.

* `DB_URI` should be of the form `mongodb://[username:password@]host[:port]`. \
  Note that the default port for mongodb is 27017. For testing this will likely work:
  `mongodb://localhost:27017`
* `DB_NAME` is the name of the database and can be pretty much any string as you would
  expect.