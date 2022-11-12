#!/usr/bin/python3

# sudo pip3 install pyarrow fastparquet

import os
import sqlite3
import sys

import pandas as pd

def main():
    in_parquet = sys.argv[1]
    base, ext = os.path.splitext(in_parquet)
    out_csv = "{}.csv".format(base)
    out_sqlite = "{}.sqlite".format(base)
    df = pd.read_parquet(in_parquet)
    df.to_csv(out_csv, index=False)
    conn = sqlite3.connect(out_sqlite)
    df.to_sql("trip", conn, if_exists="replace", index=None)
    conn.close()

main()    

# レコードのサンプル
# VendorID 1
# tpep_pickup_datetime	2022-01-01 00:35:40
# tpep_dropoff_datetime	2022-01-01 00:53:29
# passenger_count	2
# trip_distance	3.8
# RatecodeID	1
# store_and_fwd_flag	N
# PULocationID	142
# DOLocationID	236
# payment_type	1
# fare_amount	14.5
# extra	3
# mta_tax	0.5
# tip_amount	3.65
# tolls_amount	0
# improvement_surcharge	0.3
# total_amount	21.95
# congestion_surcharge	2.5
# airport_fee	0


