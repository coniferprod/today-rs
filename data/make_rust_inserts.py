# Makes SQLite INSERT statements for Rust releases from a CSV file.
# Execute the results in the SQLite shell to populate the database.

import csv

create_queries = """
CREATE TABLE IF NOT EXISTS event(
    -- alias for auto-incrementing ROWID (see https://www.sqlite.org/autoinc.html)
    event_id INTEGER PRIMARY KEY, 

    event_date DATE NOT NULL,
    event_description TEXT NOT NULL,
    category_id INTEGER NOT NULL,

    -- See https://www.sqlite.org/foreignkeys.html
    FOREIGN KEY (category_id) REFERENCES category(category_id)
);

CREATE TABLE IF NOT EXISTS category(
    category_id INTEGER PRIMARY KEY,

    primary_name TEXT NOT NULL,
    secondary_name TEXT   -- is allowed to be NULL!
);
"""

print(create_queries)

# Clear the old events and categories from the database.
print("delete from event;")
print("delete from category;")
print()

# Insert the only category we need
print("insert into category (category_id, primary_name, secondary_name) values (1, 'programming', 'rust');")
print()

input_filename = 'rust_raw.csv'
with open(input_filename) as csv_file:
    csv_reader = csv.reader(csv_file, delimiter=',')
    for row in csv_reader:
        date_string = row[0]
        version_string = row[1]
        description = f'Rust {version_string} released'
        print('insert into event (event_date, event_description, category_id)'
              + f" values ('{date_string}', '{description}', 1);")
