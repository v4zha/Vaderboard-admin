import {
    AppBar,
    Box,
    Button,
    Container,
    Drawer,
    IconButton,
    List,
    ListItem,
    ListItemText,
    Toolbar,
    Typography,
} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import UserList from "./UserList";
import { apiUrl, userCurFtsUrl } from "../utils/apiUtils";
import { EventInfo, EventState } from "../Types";

interface UserEventProps {
    eventInfo: EventInfo;
}

enum UserEventOpts {
    EventInfo = "EventInfo",
    User = "Users",
}

const UserEvent: React.FC<UserEventProps> = ({
    eventInfo,
}: UserEventProps): JSX.Element => {
    const navigate = useNavigate();
    const [opt, setOpt] = useState<UserEventOpts>(UserEventOpts.EventInfo);
    const [drawerOpen, setDrawerOpen] = useState<boolean>(false);
    const [eventState, setEventState] = useState<EventState>(eventInfo.state);
    console.log("User event , Event State : ",eventState);
    const getContent = (opt: UserEventOpts): JSX.Element => {
        switch (opt) {
            case UserEventOpts.EventInfo: {
                return (
                    <>
                        <Typography
                            variant="h3"
                            color="inherit"
                            component="div"
                        >
                            {eventInfo.name}
                        </Typography>
                        <Button
                            onClick={() => {
                                if (eventState == EventState.Stop) {
                                    navigate("/home");
                                } else {
                                    setStateChange(eventState + 1);
                                }
                            }}
                        >
                            {EventState[eventState + 1]}
                        </Button>
                    </>
                );
            }
            case UserEventOpts.User: {
                return (
                    <Container>
                        <Button
                            onClick={() => {
                                navigate("/user/add");
                            }}
                        >
                            Add User
                        </Button>
                        <UserList
                            url={userCurFtsUrl}
                            updateScore={eventState == EventState.Start}
                        />
                    </Container>
                );
            }
        }
    };

    const setStateChange = async (newState: EventState) => {
        let reqUrl: string = "";
        switch (newState) {
            case EventState.Start: {
                reqUrl = `${apiUrl}/admin/event/start`;
                break;
            }
            case EventState.Stop: {
                reqUrl = `${apiUrl}/admin/event/stop`;
                break;
            }
        }
        const res = await fetch(reqUrl, {
            method: "POST",
        });
        if (res.ok) {
            setEventState(newState);
        }
    };

    return (
        <Container>
            <Box sx={{ flexGrow: 1 }}>
                <AppBar position="static">
                    <Toolbar variant="dense">
                        <IconButton
                            edge="start"
                            color="inherit"
                            aria-label="menu"
                            onClick={() => {
                                setDrawerOpen(true);
                            }}
                            sx={{
                                mr: 2,
                                ...(drawerOpen && { display: "none" }),
                            }}
                        >
                            <MenuIcon />
                        </IconButton>
                        <Typography variant="h6" component="div">
                            {opt}
                        </Typography>
                    </Toolbar>
                </AppBar>
                <Button
                    onClick={() => {
                        navigate("/home");
                    }}
                >
                    Go Home
                </Button>
                <Drawer
                    sx={{
                        width: 240,
                        flexShrink: 0,
                        "& .MuiDrawer-paper": {
                            width: 240,
                            boxSizing: "border-box",
                            backgroundColor: "primary.main",
                        },
                    }}
                    variant="persistent"
                    anchor="left"
                    open={drawerOpen}
                >
                    <IconButton
                        onClick={() => {
                            setDrawerOpen(false);
                        }}
                    >
                        <ChevronLeftIcon />
                    </IconButton>
                    <List sx={{ mr: "2rem" }}>
                        <ListItem
                            onClick={() => {
                                setOpt(UserEventOpts.EventInfo);
                                setDrawerOpen(false);
                            }}
                        >
                            <ListItemText
                                primary="Event Info"
                                color="text.primary"
                            />
                        </ListItem>
                        <ListItem
                            onClick={() => {
                                setOpt(UserEventOpts.User);
                                setDrawerOpen(false);
                            }}
                        >
                            <ListItemText
                                primary="Users"
                                color="text.primary"
                            />
                        </ListItem>
                    </List>
                </Drawer>
            </Box>
            {getContent(opt)}
        </Container>
    );
};

export default UserEvent;
