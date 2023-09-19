CREATE TABLE IF NOT EXISTS users (
	name varchar(255),
	client_id varchar(255),
	score_matrix varchar(750),
	points int
);

CREATE TABLE IF NOT EXISTS drivers (
	id int,
	name varchar(255),
	team varchar(255),
	is_active int,
	score int
);

CREATE TABLE IF NOT EXISTS races (
	race_name varchar(255),
	score_matrix varchar(750)
);