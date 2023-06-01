import {
    List,
    ListItem,
    ListItemText,
    IconButton,
    TextField,
    Container,
} from "@mui/material";
import FileCopyIcon from "@mui/icons-material/FileCopy";
import { useEffect, useState } from "react";
import { getUsers } from "../utils/apiUtils";

const UserList = (): JSX.Element => {
    const [users, setUsers] = useState<Array<UserInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();
    useEffect(() => {
        (async () => {
            const u = await getUsers();
            setUsers(u);
        })();
    }, []);

    useEffect(() => {
        const ws = new WebSocket("ws://localhost:8080/user/fts");
        ws.addEventListener("open", () => {
            console.log("ws connected.");
        });

        ws.addEventListener("message", (event) => {
            const data: Array<UserInfo> = JSON.parse(event.data);
            setUsers(data);
        });

        setSocket(ws);
        return () => {
            ws.close();
        };
    }, []);
    const handleClick = async (userId: string) => {
        try {
            await navigator.clipboard.writeText(userId);
            console.log("User ID copied:", userId);
        } catch (error) {
            console.error("Failed to copy user ID to clipboard:", error);
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
                label="Search Users"
                variant="outlined"
                fullWidth
                onChange={handleSearch}
            />
            <List>
                {users.map((user) => (
                    <ListItem key={user.id}>
                        <ListItemText
                            primary={user.name}
                            secondary={`${user.score} - ${user.id}`}
                        />
                        <IconButton
                            onClick={() => handleClick(user.id)}
                            aria-label="Copy User ID"
                        >
                            <FileCopyIcon />
                        </IconButton>
                    </ListItem>
                ))}
            </List>
        </Container>
    );
};

export default UserList;
