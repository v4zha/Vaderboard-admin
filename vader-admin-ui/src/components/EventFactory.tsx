import { Button, Container, Typography } from "@mui/material";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import UserEvent from "./UserEvent";
import TeamEvent from "./TeamEvent";

interface FactoryProps {
    eventInfo: EventInfo;
}
enum EventState {
    Added,
    Start,
    Stop,
}

const EventFactory: React.FC<FactoryProps> = (
    props: FactoryProps
): JSX.Element => {
    const navigate = useNavigate();
    const [eventState, setEstate] = useState<EventState>(EventState.Added);
    const getEventFromType = (eventInfo: EventInfo): JSX.Element => {
        return eventInfo.event_type === "UserEvent" ? (
            <UserEvent eventInfo={eventInfo} />
        ) : (
            <TeamEvent eventInfo={eventInfo} />
        );
    };
    const setStateChange = async (eState: EventState) => {
        let reqUrl: string = "";
        switch (eState) {
            case EventState.Start: {
                reqUrl = "http://localhost:8080/admin/event/start";
                break;
            }
            case EventState.Stop: {
                reqUrl = "http://localhost:8080/admin/event/stop";
                break;
            }
        }
        const res = await fetch(reqUrl, {
            method: "POST",
        });
        if (res.ok) {
            setEstate(eState);
        }
    };

    const getStateButton = (eventState: EventState): JSX.Element => {
        if (eventState === EventState.Stop) {
            return (
                <Button
                    onClick={() => {
                        navigate("/home");
                    }}
                >
                    Go Back
                </Button>
            );
        } else {
            return (
                <Button
                    onClick={async () => {
                        await setStateChange(eventState + 1);
                    }}
                >
                    {EventState[eventState + 1]}
                </Button>
            );
        }
    };
    return (
        <Container>
            <Typography variant="h3" color="inherit" component="div">
                {props.eventInfo.name}
            </Typography>
            {getStateButton(eventState)}
            {getEventFromType(props.eventInfo)}
        </Container>
    );
};
export default EventFactory;
