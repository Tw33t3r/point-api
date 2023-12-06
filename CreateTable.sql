CREATE TABLE IF NOT EXISTS `orders`(
  `id` TEXT PRIMARY KEY NOT NULL,
  `paid` INTEGER NOT NULL,
  `currency` TEXT NOT NULL,
  `customerEmail` TEXT NOT NULL,
  `percentage` INTEGER,
  FOREIGN KEY(customerEmail) REFERENCES customers(email)
);

CREATE TABLE IF NOT EXISTS `customers`(
  `email` TEXT PRIMARY KEY NOT NULL,
  `phone` TEXT,
  `points` INTEGER NOT NULL
);


INSERT INTO `customers`(email, points)
VALUES('camenisch@hotmail.com', 123333),
('padme@msn.com',123333),
('seanq@icloud.com',123333),
('lstein@gmail.com', 123333),
('tmccarth@comcast.net',123333),
('pgolle@aol.com', 123333),
('grady@msn.com', 123333),
('scarolan@msn.com', 123333),
('mddallara@msn.com', 123333),
('dieman@me.com', 123333),
('solomon@msn.com', 123333),
('bruck@yahoo.ca',123333);
