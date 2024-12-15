CREATE TABLE IF NOT EXISTS UserData (
    ID bigserial PRIMARY KEY,
    FirstName varchar(64),
    MiddleName varchar(64),
    LastName varchar(64),
    Email varchar(128) UNIQUE NOT NULL,
    LastUpdate TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO UserData ( FirstName, LastName, Email)
VALUES 
    ( 'Buggs', 'Bunny', 'BuggsBunny@lt.com'),
    ( 'Charlie', 'Dog', 'CharlieDog@lt.com' ),
    ( 'Elmer', 'Fudd', 'ElmerFudd@lt.com' ),
    ( 'Road', 'Runner', 'meep@meep.com' ),
    ( 'Speedy', 'Gonzalez', 'SpeedyG@lt.com' )
ON CONFLICT (Email) DO NOTHING;


INSERT INTO UserData ( FirstName, MiddleName, LastName, Email)
VALUES 
    ( 'Marvin', 'the', 'Martian', 'MarvinMartian@lt.com' )
ON CONFLICT (Email) 
DO NOTHING;