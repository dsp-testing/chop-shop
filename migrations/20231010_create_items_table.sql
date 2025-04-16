CREATE TABLE items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    estimated_ship_date TEXT NOT NULL,
    supplier TEXT NOT NULL
);

INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES
-- Brake Parts (5 entries)
('Brake Pad Set - Front', 29.99, '2025-04-20', 'Rustic Brakes Inc'),
('Brake Pad Set - Rear', 27.50, '2025-04-22', 'Rustic Brakes Inc'),
('Brake Rotor - Premium', 49.99, '2025-04-25', 'Detroit Auto Parts'),
('Brake Caliper Assembly', 79.99, '2025-04-28', 'Motor City Supplies'),
('Brake Line Kit - Universal', 24.95, '2025-05-01', 'Rustic Brakes Inc'),

-- Oil Parts (4 entries)
('Oil Filter - Standard', 9.99, '2025-04-19', 'LubeMaster Co'),
('Oil Filter - Heavy Duty', 15.99, '2025-04-21', 'Detroit Filtration'),
('Synthetic Oil 5W-30 (5qt)', 32.50, '2025-04-18', 'LubeMaster Co'),
('Oil Drain Plug Kit', 8.75, '2025-04-20', 'Hubbers Supply Co'),

-- Air Filters (4 entries)
('Air Filter - Standard', 19.99, '2025-04-22', 'Detroit Filtration'),
('Air Filter - Performance', 29.99, '2025-04-24', 'TurboBreath Systems'),
('Cabin Air Filter', 22.50, '2025-04-23', 'Detroit Filtration'),
('Air Filter Box Assembly', 45.75, '2025-04-30', 'TurboBreath Systems'),

-- Spark Parts (5 entries)
('Spark Plug - Standard', 4.99, '2025-04-19', 'SparkTech Industries'),
('Spark Plug - Platinum', 7.99, '2025-04-20', 'SparkTech Industries'),
('Spark Plug - Iridium', 12.99, '2025-04-22', 'Premium Ignition Co'),
('Spark Plug Wire Set', 29.99, '2025-04-25', 'SparkTech Industries'),
('Ignition Coil Pack', 45.50, '2025-04-28', 'Premium Ignition Co'),

-- Battery Parts (4 entries)
('Battery - Standard 12V', 89.99, '2025-04-21', 'PowerCell Inc'),
('Battery - Heavy Duty', 119.99, '2025-04-23', 'PowerCell Inc'),
('Battery Terminal Connectors', 12.50, '2025-04-20', 'Electric Supply Detroit'),
('Battery Charger 12V', 49.99, '2025-04-25', 'Electric Supply Detroit'),

-- Engine Electrical (5 entries)
('Alternator - Remanufactured', 149.99, '2025-04-26', 'Electric Supply Detroit'),
('Alternator - New', 199.99, '2025-04-28', 'PowerGen Solutions'),
('Voltage Regulator', 45.75, '2025-04-22', 'Electric Supply Detroit'),
('Alternator Belt', 25.99, '2025-04-20', 'BeltMaster Detroit'),
('Alternator Pulley', 19.99, '2025-04-23', 'PowerGen Solutions'),

-- Cooling System (5 entries)
('Radiator - Aluminum', 199.99, '2025-04-29', 'CoolFlow Systems'),
('Radiator - OEM Replacement', 175.50, '2025-05-01', 'CoolFlow Systems'),
('Radiator Hose Set', 39.99, '2025-04-24', 'Hubbers Supply Co'),
('Coolant Reservoir Tank', 29.95, '2025-04-22', 'CoolFlow Systems'),
('Radiator Cap - High Pressure', 12.99, '2025-04-20', 'Hubbers Supply Co'),

-- Fuel System (5 entries)
('Fuel Pump Assembly', 129.99, '2025-04-28', 'FuelTech Solutions'),
('Fuel Filter', 24.99, '2025-04-23', 'Detroit Filtration'),
('Fuel Injector Set', 199.95, '2025-05-02', 'FuelTech Solutions'),
('Fuel Pressure Regulator', 59.99, '2025-04-25', 'FuelTech Solutions'),
('Fuel Tank Strap Kit', 45.50, '2025-04-27', 'Hubbers Supply Co'),

-- Starter Components (4 entries)
('Starter Motor - Remanufactured', 99.99, '2025-04-25', 'PowerStart Detroit'),
('Starter Motor - New', 149.99, '2025-04-28', 'PowerStart Detroit'),
('Starter Solenoid', 24.95, '2025-04-22', 'Electric Supply Detroit'),
('Starter Drive Assembly', 39.99, '2025-04-26', 'PowerStart Detroit'),

-- Engine Belts (4 entries)
('Timing Belt Kit', 49.99, '2025-04-24', 'BeltMaster Detroit'),
('Serpentine Belt', 29.99, '2025-04-21', 'BeltMaster Detroit'),
('Drive Belt - AC', 19.99, '2025-04-20', 'BeltMaster Detroit'),
('Belt Tensioner Assembly', 45.75, '2025-04-23', 'Motor City Supplies'),

-- Engine Water Components (4 entries)
('Water Pump - Standard', 79.99, '2025-04-27', 'CoolFlow Systems'),
('Water Pump - Heavy Duty', 109.99, '2025-04-30', 'CoolFlow Systems'),
('Water Outlet Fitting', 15.50, '2025-04-23', 'Hubbers Supply Co'),
('Thermostat Housing Assembly', 34.95, '2025-04-25', 'CoolFlow Systems'),

-- Lights (5 entries)
('Headlight Assembly - Driver Side', 99.99, '2025-04-26', 'VisionBright Lighting'),
('Headlight Assembly - Passenger Side', 99.99, '2025-04-26', 'VisionBright Lighting'),
('Headlight Bulb - LED Upgrade', 39.99, '2025-04-22', 'VisionBright Lighting'),
('Fog Light Kit - Universal', 75.50, '2025-04-28', 'VisionBright Lighting'),
('Headlight Switch', 29.95, '2025-04-24', 'Electric Supply Detroit'),

-- Tail Lights (4 entries)
('Tail Light Assembly - Driver Side', 64.99, '2025-04-27', 'VisionBright Lighting'),
('Tail Light Assembly - Passenger Side', 64.99, '2025-04-27', 'VisionBright Lighting'),
('Tail Light Bulb Set - LED', 34.99, '2025-04-23', 'VisionBright Lighting'),
('Brake Light Switch', 19.95, '2025-04-22', 'Electric Supply Detroit'),

-- Wiper Components (4 entries)
('Windshield Wiper Blade Set - Premium', 34.99, '2025-04-22', 'ClearView Wipers'),
('Windshield Wiper Blade Set - Standard', 24.99, '2025-04-20', 'ClearView Wipers'),
('Wiper Motor Assembly', 79.95, '2025-04-25', 'Electric Supply Detroit'),
('Windshield Washer Pump', 19.99, '2025-04-21', 'ClearView Wipers'),

-- Exhaust Components (5 entries)
('Muffler - Performance', 89.99, '2025-04-29', 'RumbleTone Exhaust'),
('Muffler - Standard', 59.99, '2025-04-27', 'RumbleTone Exhaust'),
('Catalytic Converter - Universal', 149.99, '2025-05-02', 'RumbleTone Exhaust'),
('Exhaust Pipe Section', 45.75, '2025-04-25', 'RumbleTone Exhaust'),
('Exhaust Hanger Kit', 19.95, '2025-04-23', 'Hubbers Supply Co'),

-- Transmission Parts (5 entries)
('Transmission Fluid Pan Kit', 39.99, '2025-04-24', 'GearMaster Trans'),
('Transmission Filter Kit', 24.95, '2025-04-22', 'Detroit Filtration'),
('Automatic Transmission Solenoid', 59.99, '2025-04-26', 'GearMaster Trans'),
('Transmission Cooler Line Set', 35.50, '2025-04-25', 'CoolFlow Systems'),
('Transmission Mount', 49.99, '2025-04-27', 'Motor City Supplies'),

-- Door Components (5 entries)
('Door Handle - Exterior (Driver)', 45.99, '2025-04-23', 'BodyParts Detroit'),
('Door Handle - Exterior (Passenger)', 45.99, '2025-04-23', 'BodyParts Detroit'),
('Door Lock Actuator', 34.95, '2025-04-25', 'Electric Supply Detroit'),
('Power Window Motor', 69.99, '2025-04-27', 'Electric Supply Detroit'),
('Door Hinge Set', 29.95, '2025-04-24', 'BodyParts Detroit'),

-- Interior Parts (5 entries)
('Seat Cover Set - Universal', 89.99, '2025-04-26', 'ComfortRide Interiors'),
('Steering Wheel Cover - Leather', 29.95, '2025-04-22', 'ComfortRide Interiors'),
('Floor Mat Set - All Weather', 59.99, '2025-04-24', 'ComfortRide Interiors'),
('Seat Belt Replacement', 45.75, '2025-04-25', 'BodyParts Detroit'),
('Dashboard Cover - Custom Fit', 49.99, '2025-04-28', 'ComfortRide Interiors'),

-- Wheel Components (5 entries)
('Wheel Rim - Alloy 17"', 129.99, '2025-04-29', 'RoadKing Wheels'),
('Wheel Hub Assembly', 89.95, '2025-04-27', 'Motor City Supplies'),
('Lug Nut Set - Chrome', 19.99, '2025-04-22', 'RoadKing Wheels'),
('Wheel Bearing Kit', 45.50, '2025-04-25', 'Motor City Supplies'),
('Wheel Spacers - 1.5"', 59.99, '2025-04-26', 'RoadKing Wheels');
