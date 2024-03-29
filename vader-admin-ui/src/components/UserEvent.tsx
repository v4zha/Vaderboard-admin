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
import { apiUrl, userCurFtsUrl } from "../utils/ApiUtils";
import { EventInfo, EventState, UserEventOpts } from "../Types";

interface UserEventProps {
    eventInfo: EventInfo;
    drawerOpt: UserEventOpts;
}

export const UserEvent: React.FC<UserEventProps> = ({
    eventInfo,
    drawerOpt,
}: UserEventProps): JSX.Element => {
    const navigate = useNavigate();
    const [opt, setOpt] = useState<UserEventOpts>(drawerOpt);
    const [drawerOpen, setDrawerOpen] = useState<boolean>(false);
    const [eventState, setEventState] = useState<EventState>(eventInfo.state);
    const GetContent: React.FC<{ opt: UserEventOpts }> = ({
        opt,
    }: {
        opt: UserEventOpts;
    }): JSX.Element => {
        switch (opt) {
            case UserEventOpts.EventDetails: {
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
                            updateScore={eventState === EventState.Start}
                        />
                    </Container>
                );
            }
            default: {
                return <></>;
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
                                setOpt(UserEventOpts.EventDetails);
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
            {GetContent({ opt })}
        </Container>
    );
};
