# FILES

data/ --- get from this page https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page
  parquet_to_csv.py --- convert parquet to csv and sqlite3
  
# Data

There seem four slightly different kinds of data. 
We'll see which one is most suitable.

* Yellow Taxi Trip Records (PARQUET)
* Green Taxi Trip Records (PARQUET)
* For-Hire Vehicle Trip Records (PARQUET)
* High Volume For-Hire Vehicle Trip Records (PARQUET)

* https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2022-01.parquet
* https://d37ci6vzurychx.cloudfront.net/trip-data/green_tripdata_2022-01.parquet
* https://d37ci6vzurychx.cloudfront.net/trip-data/fhv_tripdata_2022-01.parquet
* https://d37ci6vzurychx.cloudfront.net/trip-data/fhvhv_tripdata_2022-01.parquet
  
# Convert

```
./parquet_to_csv.py ???.parquet 
```

will output ???.csv and ???.sqlite
