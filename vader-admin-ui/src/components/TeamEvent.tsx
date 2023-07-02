import React, { useState } from "react";
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
import { useNavigate } from "react-router-dom";
import TeamList from "./TeamList";
import { apiUrl, teamCurFtsUrl } from "../utils/apiUtils";
import { EventInfo, EventState, TeamEventOpts } from "../Types";

interface TeamEventProps {
    eventInfo: EventInfo;
    drawerOpt: TeamEventOpts;
}

export const TeamEvent: React.FC<TeamEventProps> = ({
    eventInfo,
    drawerOpt,
}: TeamEventProps): JSX.Element => {
    const navigate = useNavigate();
    const [opt, setOpt] = useState<TeamEventOpts>(drawerOpt);
    const [drawerOpen, setDrawerOpen] = useState<boolean>(false);
    const [eventState, setEventState] = useState<EventState>(eventInfo.state);
    const GetContent = ({ opt }: { opt: TeamEventOpts }): JSX.Element => {
        switch (opt) {
            case TeamEventOpts.EventDetails:
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
                                if (eventState === EventState.Stop) {
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
            case TeamEventOpts.TeamList:
                return (
                    <Container>
                        <Button
                            onClick={() => {
                                navigate("/team/add", {
                                    state: {
                                        maxTeamSize: (
                                            eventInfo.event_type as {
                                                TeamEvent: {
                                                    team_size: number;
                                                };
                                            }
                                        ).TeamEvent.team_size,
                                    },
                                });
                            }}
                        >
                            Add Team
                        </Button>
                        <TeamList
                            url={teamCurFtsUrl}
                            updateScore={eventState == EventState.Start}
                        />
                    </Container>
                );
            default:
                return <></>;
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
                                setOpt(TeamEventOpts.EventDetails);
                                setDrawerOpen(false);
                            }}
                        >
                            <ListItemText
                                primary={TeamEventOpts.EventDetails}
                            />
                        </ListItem>
                        <ListItem
                            onClick={() => {
                                setOpt(TeamEventOpts.TeamList);
                                setDrawerOpen(false);
                            }}
                        >
                            <ListItemText primary={TeamEventOpts.TeamList} />
                        </ListItem>
                    </List>
                </Drawer>
            </Box>
            {GetContent({ opt })}
        </Container>
    );
};
