import {
    Button,
    Container,
    FormControl,
    InputLabel,
    MenuItem,
    Select,
    TextField,
    Typography,
} from "@mui/material";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

enum EventTypeOpt {
    UserEvent = "UserEvent",
    TeamEvent = "TeamEvent",
}
const AddEvent: React.FC = (): JSX.Element => {
    const [eventType, setEventType] = useState<EventTypeOpt>(
        EventTypeOpt.UserEvent
    );
    const [teamSize, setTeamSize] = useState<number>(0);
    const [eventName, setEventName] = useState<string>("");
    const navigate = useNavigate();
    const teamSizeSel = (eventType: EventTypeOpt): JSX.Element => {
        if (eventType === EventTypeOpt.TeamEvent) {
            return (
                <TextField
                    id="team-size"
                    label="Team Size"
                    type="number"
                    inputProps={{ min: 1 }}
                    variant="outlined"
                    fullWidth
                    value={teamSize}
                    onChange={(e) => setTeamSize(Number(e.target.value))}
                />
            );
        } else {
            return <></>;
        }
    };
    const eventSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const url = "http://localhost:8080/admin/event/add";
        let data;
        let headers = new Headers();
        headers.append("Content-Type", "application/json");
        if (eventType === EventTypeOpt.TeamEvent && teamSize > 0) {
            data = {
                name: eventName,
                event_type: { TeamEvent: { team_size: teamSize } },
            };
        } else if (eventType === EventTypeOpt.UserEvent) {
            data = {
                name: eventName,
                event_type: "UserEvent",
            };
        }
        console.log("Sending data : ", JSON.stringify(data));
        const res = await fetch(url, {
            method: "POST",
            headers,
            body: JSON.stringify(data),
        });
        console.log(res);
        if (res.ok) {
            navigate("/event");
        }
    };
    return (
        <Container maxWidth="sm">
            <div style={{ textAlign: "center", marginTop: "3rem" }}>
                <Typography variant="h4" color="inherit" component="div">
                    VaderBoard
                </Typography>
            </div>
            <div className="login-container">
                <Typography variant="h6" color="inherit" component="div">
                    Event Registration
                </Typography>
                <form onSubmit={eventSubmit} className="form-container">
                    <TextField
                        id="event-name"
                        label="Event Name"
                        type="text"
                        variant="outlined"
                        fullWidth
                        value={eventName}
                        onChange={(e) => setEventName(e.target.value)}
                    />
                    <FormControl variant="filled" sx={{ m: 1, minWidth: 120 }}>
                        <InputLabel id="event-type">Event Type</InputLabel>
                        <Select
                            labelId="event-type-label"
                            id="event-type-label"
                            value={eventType}
                            label="event-type"
                            onChange={(e) =>
                                setEventType(e.target.value as EventTypeOpt)
                            }
                        >
                            <MenuItem value={EventTypeOpt.UserEvent}>
                                UserEvent
                            </MenuItem>
                            <MenuItem value={EventTypeOpt.TeamEvent}>
                                TeamEvent
                            </MenuItem>
                        </Select>
                    </FormControl>
                    {teamSizeSel(eventType)}
                    <Button
                        type="submit"
                        variant="contained"
                        color="primary"
                        fullWidth
                    >
                        Add Event
                    </Button>
                </form>
            </div>
        </Container>
    );
};
export default AddEvent;
