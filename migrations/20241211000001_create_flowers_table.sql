-- Create flowers table
CREATE TABLE IF NOT EXISTS flowers (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    color VARCHAR(50) NOT NULL,
    description TEXT,
    price DOUBLE PRECISION NOT NULL DEFAULT 0,
    stock INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_flowers_name ON flowers (name);

CREATE INDEX IF NOT EXISTS idx_flowers_color ON flowers (color);

CREATE INDEX IF NOT EXISTS idx_flowers_created_at ON flowers (created_at DESC);

-- Add some sample data
INSERT INTO
    flowers (
        id,
        name,
        color,
        description,
        price,
        stock
    )
VALUES (
        '550e8400-e29b-41d4-a716-446655440001',
        'Rose',
        'red',
        'A beautiful red rose, symbol of love and passion',
        25000,
        100
    ),
    (
        '550e8400-e29b-41d4-a716-446655440002',
        'Tulip',
        'yellow',
        'Bright yellow tulip from Holland',
        20000,
        75
    ),
    (
        '550e8400-e29b-41d4-a716-446655440003',
        'Sunflower',
        'yellow',
        'Large sunflower that follows the sun',
        15000,
        50
    ),
    (
        '550e8400-e29b-41d4-a716-446655440004',
        'Orchid',
        'purple',
        'Elegant purple orchid for special occasions',
        75000,
        30
    ),
    (
        '550e8400-e29b-41d4-a716-446655440005',
        'Lily',
        'white',
        'Pure white lily with a sweet fragrance',
        35000,
        60
    ),
    (
        '550e8400-e29b-41d4-a716-446655440006',
        'Jasmine',
        'white',
        'Small white jasmine flowers with intense aroma',
        18000,
        90
    ),
    (
        '550e8400-e29b-41d4-a716-446655440007',
        'Lavender',
        'purple',
        'Calming lavender from Provence',
        22000,
        80
    ),
    (
        '550e8400-e29b-41d4-a716-446655440008',
        'Daisy',
        'white',
        'Simple and cheerful white daisy',
        12000,
        120
    ),
    (
        '550e8400-e29b-41d4-a716-446655440009',
        'Carnation',
        'pink',
        'Pink carnation for mothers day',
        18000,
        95
    ),
    (
        '550e8400-e29b-41d4-a716-446655440010',
        'Hydrangea',
        'blue',
        'Beautiful blue hydrangea cluster',
        45000,
        40
    );