import pandas
import re
import matplotlib.pyplot as plt

# 乗った時間帯ごとの、平均乗車距離
# 例えば、昼乗る人より夜乗る人の方が長距離を移動しているのか？、という結果が見たい
trip_record = pandas.read_csv("../tlc_trip_record/data/yellow_tripdata_2022-10.csv")
trip_record = trip_record[['tpep_pickup_datetime', 'tpep_dropoff_datetime', 'passenger_count', 'trip_distance']]
trip_record['pickup_hours'] = trip_record['tpep_pickup_datetime'].apply(lambda x: re.sub(r' ([0-9][0-9]):', '\1', x))
trip_record['dropoff_hours'] = trip_record['tpep_pickup_datetime'].apply(lambda x: re.sub(r' ([0-9][0-9]):', '\1', x))
print(trip_record)

#plt.plot(trip_record['pickup_hours'], trip_record['dropoff_hours'])
#plt.show()
