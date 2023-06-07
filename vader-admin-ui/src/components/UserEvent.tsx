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
    const getContent = (opt: UserEventOpts): JSX.Element => {
        switch (opt) {
            case UserEventOpts.EventInfo: {
                return <></>;
            }
            case UserEventOpts.User: {
                return <></>;
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
                <Button
                    onClick={() => {
                        navigate("/event");
                    }}
                >
                    add Event
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
                    <List
                        sx={{
                            mr: "2rem",
                        }}
                    >
                        <ListItem
                            onClick={() => {
                                setOpt(UserEventOpts.EventInfo);
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
