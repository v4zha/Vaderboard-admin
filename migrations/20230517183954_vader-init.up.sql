CREATE TABLE events (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE,
    logo TEXT ,
    event_type TEXT
);

CREATE TABLE teams (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE,
    score INTEGER,
    logo TEXT
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE,
    score INTEGER,
    logo TEXT
);

CREATE TABLE event_teams (
    event_id UUID,
    team_id UUID,
    PRIMARY KEY (event_id, team_id),
    FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE
);

CREATE TABLE team_members (
    team_id UUID,
    user_id UUID,
    PRIMARY KEY (team_id, user_id),
    FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    UNIQUE (user_id)
);

CREATE TABLE event_users (
    user_id UUID,
    event_id UUID,
    PRIMARY KEY (user_id, event_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE
);


CREATE TABLE admin_login (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT UNIQUE,
  password TEXT UNIQUE
);

-- create Virtual tables for FTS5 : )
CREATE VIRTUAL TABLE events_fts USING FTS5(id,name,logo,event_type,content='events');
CREATE VIRTUAL TABLE users_fts USING FTS5(id,name,score,logo,content='users');
CREATE VIRTUAL TABLE teams_fts USING FTS5(id,name,score,logo,content='teams');

-- Create Trigger to update Virtual table on CRUD

-- Insertion Trigger

CREATE TRIGGER events_fts_insert AFTER INSERT ON events BEGIN
   INSERT INTO events_fts(id,name,logo,event_type) VALUES(new.id,new.name,new.logo,new.event_type);
END;

CREATE TRIGGER users_fts_insert AFTER INSERT ON users BEGIN
   INSERT INTO users_fts(id,name,score,logo) VALUES(new.id,new.name,new.score,new.logo);
END;

CREATE TRIGGER teams_fts_insert AFTER INSERT ON teams BEGIN
   INSERT INTO teams_fts(id,name,score,logo) VALUES(new.id,new.name,new.score,new.logo);
END;

-- Updation Trigger

CREATE TRIGGER users_fts_update AFTER UPDATE OF score ON users BEGIN
  UPDATE users_fts SET score = new.score WHERE id=old.id;
END;

CREATE TRIGGER teams_fts_update AFTER UPDATE OF score ON teams BEGIN
  UPDATE teams_fts SET score = new.score WHERE id=old.id;
END;

-- Deletion Trigger

CREATE TRIGGER events_fts_delete AFTER DELETE ON events BEGIN
  DELETE FROM events_fts WHERE id=old.id;
END;

CREATE TRIGGER users_fts_delete AFTER DELETE ON users BEGIN
  DELETE FROM  users_fts WHERE id=old.id;
END;

CREATE TRIGGER teams_fts_delete AFTER DELETE ON teams BEGIN
  DELETE FROM teams_fts WHERE id=old.id;
END;


