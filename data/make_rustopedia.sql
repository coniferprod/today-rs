
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

delete from event;
delete from category;

insert into category (category_id, primary_name, secondary_name) values (1, 'programming', 'rust');

insert into event (event_date, event_description, category_id) values ('2022-11-03', 'Rust 1.65.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2022-12-15', 'Rust 1.66.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-01-26', 'Rust 1.67.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-03-09', 'Rust 1.68.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-04-20', 'Rust 1.69.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-06-01', 'Rust 1.70.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-07-13', 'Rust 1.71.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-08-24', 'Rust 1.72.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-10-05', 'Rust 1.73.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-11-16', 'Rust 1.74.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2023-12-28', 'Rust 1.75.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-02-08', 'Rust 1.76.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-03-21', 'Rust 1.77.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-05-02', 'Rust 1.78.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-06-13', 'Rust 1.79.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-07-25', 'Rust 1.80.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-09-05', 'Rust 1.81.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-10-17', 'Rust 1.82.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2024-11-28', 'Rust 1.83.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-01-09', 'Rust 1.84.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-02-20', 'Rust 1.85.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-04-03', 'Rust 1.86.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-05-15', 'Rust 1.87.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-06-26', 'Rust 1.88.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-08-08', 'Rust 1.89.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-09-18', 'Rust 1.90.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-10-30', 'Rust 1.91.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2025-12-11', 'Rust 1.92.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2026-01-22', 'Rust 1.93.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2026-03-05', 'Rust 1.94.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2026-04-16', 'Rust 1.95.0 released', 1);
insert into event (event_date, event_description, category_id) values ('2026-05-28', 'Rust 1.96.0 released', 1);
