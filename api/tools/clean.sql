TRUNCATE administrator RESTART IDENTITY CASCADE;
TRUNCATE post RESTART IDENTITY CASCADE;
insert into administrator (user_name, email, password) values ('dave', 'dharric@live.com', '123');