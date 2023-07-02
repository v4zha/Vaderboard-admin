import { useState } from "react";
import { Button, Container, TextField, Typography } from "@mui/material";
import { apiUrl } from "../utils/apiUtils";
import { useNavigate } from "react-router-dom";
import { UserEventOpts } from "../Types";

export const AddUser: React.FC = () => {
    const [userName, setUserName] = useState("");
    const navigate = useNavigate();

    const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const url = `${apiUrl}/admin/event/user/add`;
        const formData = {
            name: userName,
        };
        try {
            const response = await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(formData),
            });
            if (response.ok) {
                console.log("User added successfully!");
                navigate("/event", {
                    state: { opt: UserEventOpts.User },
                });
            } else {
                console.log("Failed to add user");
            }
        } catch (error: any) {
            console.log("Error:", error.message);
        }
    };

    return (
        <Container maxWidth="sm">
            <div className="user-container">
                <Typography variant="h6" color="inherit" component="div">
                    Add User
                </Typography>
                <form onSubmit={handleFormSubmit} className="form-container">
                    <TextField
                        id="user-name"
                        label="User Name"
                        type="text"
                        variant="outlined"
                        fullWidth
                        value={userName}
                        onChange={(e) => setUserName(e.target.value)}
                    />
                    <Button
                        type="submit"
                        variant="contained"
                        color="primary"
                        fullWidth
                    >
                        Add User
                    </Button>
                </form>
            </div>
        </Container>
    );
};
