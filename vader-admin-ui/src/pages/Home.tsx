import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import Container from "@mui/material/Container";
import EventList from "../components/EventList";
import { useState } from "react";
import UserList from "../components/UserList";
import TeamList from "../components/TeamList";
import { Drawer, List, ListItem, ListItemText } from "@mui/material";

enum HomeOpts {
    Event = "Events",
    User = "Users",
    Team = "Teams",
}

const Home = (): JSX.Element => {
    const [opt, setOpt] = useState<HomeOpts>(HomeOpts.Event);
    const [drawerOpen, setDrawerOpen] = useState<boolean>(false);
    const getList = (opt: HomeOpts): JSX.Element => {
        switch (opt) {
            case HomeOpts.Event: {
                return <EventList />;
            }
            case HomeOpts.User: {
                return <UserList />;
            }
            case HomeOpts.Team: {
                return <TeamList />;
            }
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
                <Drawer
                    sx={{
                        width: 240,
                        flexShrink: 0,
                    }}
                    variant="persistent"
                    anchor="left"
                    open={drawerOpen}
                    color="background.default"
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
                                setOpt(HomeOpts.Event);
                            }}
                        >
                            <ListItemText
                                primary="Events"
                                color="text.primary"
                            />
                        </ListItem>
                        <ListItem
                            onClick={() => {
                                setOpt(HomeOpts.Team);
                            }}
                        >
                            <ListItemText
                                primary="Teams"
                                color="text.primary"
                            />
                        </ListItem>
                        <ListItem
                            onClick={() => {
                                setOpt(HomeOpts.User);
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
            {getList(opt)}
        </Container>
    );
};

export default Home;
