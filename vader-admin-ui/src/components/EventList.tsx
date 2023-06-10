import {
    List,
    ListItem,
    ListItemText,
    IconButton,
    Container,
    TextField,
} from "@mui/material";
import FileCopyIcon from "@mui/icons-material/FileCopy";
import { useEffect, useState } from "react";
import { getEvents } from "../utils/apiUtils";

interface EventListProps {
    url: string;
}

const EventList = ({ url: string }: EventListProps): JSX.Element => {
    const [events, setEvents] = useState<Array<EventInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();

    const getEventType = (event_type: EventType): string => {
        if (event_type == "UserEvent") {
            return "UserEvent";
        } else {
            return "TeamEvent";
        }
    };

    useEffect(() => {
        (async () => {
            const e = await getEvents();
            setEvents(e);
        })();
    }, []);

    useEffect(() => {
        const ws = new WebSocket("ws://localhost:8080/event/fts");
        ws.addEventListener("open", () => {
            console.log("ws connected.");
        });

        ws.addEventListener("message", (event) => {
            const data: Array<EventInfo> = JSON.parse(event.data);
            setEvents(data);
        });

        setSocket(ws);
        return () => {
            ws.close();
        };
    }, []);

    const handleClick = async (eventId: string) => {
        try {
            await navigator.clipboard.writeText(eventId);
            console.log("Event ID copied:", eventId);
        } catch (error) {
            console.error("Failed to copy event ID to clipboard:", error);
        }
    };

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        const param = event.target.value;
        if (socket) {
            socket.send(param);
        }
    };
    return (
        <Container sx={{ marginTop: "2rem" }}>
            <TextField
                label="Search Event"
                variant="outlined"
                fullWidth
                onChange={handleSearch}
            />
            <List>
                {events.map((event) => (
                    <ListItem key={event.id}>
                        <ListItemText
                            primary={event.name}
                            secondary={`${getEventType(event.event_type)} - ${
                                event.id
                            }`}
                        />
                        <IconButton
                            onClick={() => handleClick(event.id)}
                            aria-label="Copy Event ID"
                        >
                            <FileCopyIcon />
                        </IconButton>
                    </ListItem>
                ))}
            </List>
        </Container>
    );
};

export default EventList;
