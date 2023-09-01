import { Container, TextField, Button, Typography } from "@mui/material";
import { apiUrl } from "../utils/ApiUtils";
import { useState } from "react";
import "./Login.css";
import { useNavigate } from "react-router-dom";

const Login = (): JSX.Element => {
    const [username, setUsername] = useState<string>();
    const [password, setPassword] = useState<string>();
    const [invalid, setInvalid] = useState<string | null>(null);
    const navigate = useNavigate();

    const loginSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const data = {
            username: username,
            password: password,
        };
        const url = `${apiUrl}/login`;
        const res = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });
        if (res.ok) {
            navigate("/home");
        } else {
            setInvalid("Incorrect Username/Password");
        }
    };
    return (
        <div className="App">
            <Container maxWidth="sm">
                <div style={{ textAlign: "center", marginTop: "3rem" }}>
                    <Typography variant="h4" color="inherit" component="div">
                        VaderBoard
                    </Typography>
                </div>
                <div className="login-container">
                    <Typography variant="h6" color="secondary" component="div">
                        Admin Login
                    </Typography>
                    <form onSubmit={loginSubmit} className="form-container">
                        {invalid ? <div>{invalid}</div> : <></>}
                        <TextField
                            id="username"
                            label="Username"
                            type="text"
                            color="secondary"
                            variant="outlined"
                            fullWidth
                            value={username}
                            onChange={(e) => {
                                setUsername(e.target.value);
                            }}
                        />
                        <TextField
                            id="password"
                            label="Password"
                            type="password"
                            color="secondary"
                            variant="outlined"
                            fullWidth
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                        <Button
                            type="submit"
                            variant="contained"
                            color="primary"
                            fullWidth
                        >
                            Login
                        </Button>
                    </form>
                </div>
            </Container>
        </div>
    );
};
export default Login;
