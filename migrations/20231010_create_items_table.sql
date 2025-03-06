CREATE TABLE items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    estimated_ship_date TEXT NOT NULL,
    supplier TEXT NOT NULL
);

INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES
('Brake Pad', 29.99, '2023-11-01', 'Supplier A'),
('Oil Filter', 9.99, '2023-11-02', 'Supplier B'),
('Air Filter', 19.99, '2023-11-03', 'Supplier C'),
('Spark Plug', 4.99, '2023-11-04', 'Supplier D'),
('Battery', 89.99, '2023-11-05', 'Supplier E'),
('Alternator', 149.99, '2023-11-06', 'Supplier F'),
('Radiator', 199.99, '2023-11-07', 'Supplier G'),
('Fuel Pump', 129.99, '2023-11-08', 'Supplier H'),
('Starter Motor', 99.99, '2023-11-09', 'Supplier I'),
('Timing Belt', 49.99, '2023-11-10', 'Supplier J'),
('Water Pump', 79.99, '2023-11-11', 'Supplier K'),
('Headlight', 39.99, '2023-11-12', 'Supplier L'),
('Tail Light', 34.99, '2023-11-13', 'Supplier M'),
('Windshield Wiper', 14.99, '2023-11-14', 'Supplier N'),
('Muffler', 59.99, '2023-11-15', 'Supplier O');
