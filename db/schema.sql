
CREATE TABLE UserData (
    ID bigint NOT NULL UNIQUE,
    FirstName varchar(64),
    MiddleName varchar(64),
    LastName varchar(64),
    LastUpdate TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (ID)
);

INSERT INTO UserData ( FirstName, LastName)
VALUES 
( 'Buggs', 'Bunny' )
( 'Charlie', 'Dog' )
( 'Elmer', 'Fudd' )
( 'Road', 'Runner' )
( 'Speedy', 'Gonzalez' )


INSERT INTO UserData ( FirstName, MiddleName, LastName)
VALUES ( 'Marvin', 'the', 'Martian' )