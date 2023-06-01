import { Container, TextField, Button } from "@mui/material";
import { useState } from "react";
import "./Login.css";
const Login = (): JSX.Element => {
    const [username, setUsername] = useState<string>();
    const [password, setPassword] = useState<string>();

    const loginSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        let data = {
            username: username,
            password: password,
        };
        let url = "http://localhost:8080/login";
        let res = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });
        if (res.ok) {
            console.log("Login successful : ) ");
        }
    };

    return (
        <div className="App">
            <Container maxWidth="sm">
                <div style={{ textAlign: "center" }}>
                    <h1>VaderBoard</h1>
                </div>
                <div className="login-container">
                    <h2>Admin Login</h2>
                    <form onSubmit={loginSubmit} className="form-container">
                        <TextField
                            id="username"
                            label="Username"
                            type="text"
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
