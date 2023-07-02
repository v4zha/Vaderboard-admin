import {
    List,
    ListItem,
    ListItemText,
    IconButton,
    TextField,
    Container,
    Button,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
} from "@mui/material";
import FileCopyIcon from "@mui/icons-material/FileCopy";
import React, { useEffect, useState } from "react";
import { UserInfo } from "../Types";
import { apiUrl } from "../utils/ApiUtils";

interface UserListProps {
    url: string;
    updateScore?: boolean;
}

const UserList = (props: UserListProps): JSX.Element => {
    const [users, setUsers] = useState<Array<UserInfo>>([]);
    const [socket, setSocket] = useState<WebSocket>();
    const [selectedUserId, setSelectedUserId] = useState<string>("");
    const [isDialogOpen, setIsDialogOpen] = useState<boolean>(false);
    const [newScore, setNewScore] = useState<number>(0);

    useEffect(() => {
        const ws = new WebSocket(props.url);
        ws.addEventListener("open", () => {
            console.log("ws connected.");
            ws.send("");
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

    const handleScoreUpdate = (userId: string) => {
        setSelectedUserId(userId);
        setIsDialogOpen(true);
    };

    const handleScoreChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setNewScore(Number(event.target.value));
    };

    const handleSubmit = () => {
        const updatedScore = {
            id: selectedUserId,
            score: newScore,
        };

        const data = JSON.stringify(updatedScore);

        fetch(`${apiUrl}/admin/score/update`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: data,
        })
            .then((response) => {
                if (response.ok) {
                    console.log("Score updated successfully");
                    if (socket) {
                        socket.send("");
                    }
                } else {
                    console.error("Failed to update score");
                }
            })
            .catch((error) => {
                console.error("Failed to update score:", error);
            });

        setIsDialogOpen(false);
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
                        {props.updateScore && (
                            <div>
                                <Button
                                    onClick={() => handleScoreUpdate(user.id)}
                                    variant="outlined"
                                    size="small"
                                >
                                    Update Score
                                </Button>
                            </div>
                        )}
                        <IconButton
                            onClick={() => handleClick(user.id)}
                            aria-label="Copy User ID"
                        >
                            <FileCopyIcon />
                        </IconButton>
                    </ListItem>
                ))}
            </List>
            <Dialog open={isDialogOpen} onClose={() => setIsDialogOpen(false)}>
                <DialogTitle>Update Score</DialogTitle>
                <DialogContent>
                    <TextField
                        label="Current Score"
                        value={
                            users.find((user) => user.id === selectedUserId)
                                ?.score || ""
                        }
                        disabled
                    />
                    <TextField
                        label="New Score"
                        type="number"
                        value={newScore}
                        color='secondary'
                        inputProps={{ min: -99999 }}
                        onChange={handleScoreChange}

                    />
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => setIsDialogOpen(false)}>
                        Cancel
                    </Button>
                    <Button onClick={handleSubmit}>Submit</Button>
                </DialogActions>
            </Dialog>
        </Container>
    );
};

export default UserList;
