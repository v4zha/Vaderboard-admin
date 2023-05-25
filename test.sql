select 
  t.id as id ,
  t.name as name, 
  t.score as score,
  t.logo as logo,
  group_concat(tm.user_id,",") as contestants
from 
    events e
  join event_teams et on et.event_id = e.id
  join teams t on et.team_id = t.id
  join team_members tm on tm.team_id = t.id
;
