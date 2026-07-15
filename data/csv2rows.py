import sys
import csv

input_filename = sys.argv[1]
with open(input_filename) as csv_file:
    csv_reader = csv.reader(csv_file, delimiter=',')
    for row in csv_reader:
        for column in row:
            print(column)
        print()
