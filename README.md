# VaderBoard Admin Panel

VaderBoard Admin Panel is a web application built using Actix and Actix Actor WebSockets. It provides an intuitive admin panel with live leaderboard functionality. The application supports both user and team events, allowing you to manage and monitor scores in real-time.

![Vaderboard Admin](/assets/v-admin.png)

## Installation

To install and run the VaderBoard Admin Panel, follow the steps below:

1. Clone the repository:

    ```
    git clone https://github.com/v4zha/Vaderboard-admin
    ```

2. Navigate to the project directory:

    ```
    cd Vaderboard-admin
    ```

3. rename the `.sample_env` `.env` file in the project root directory and set the required environment variables. For example:

    ```
    HOST=0.0.0.0
    PORT= 8080
    DATABASE_URL=<db url>
    ADMIN_USERNAME=<enter admin username>
    ADMIN_PASSWORD=<enter admin password>
    VADERBOARD_LIMIT=10
    ```

4. Install [sqlx-cli](https://crates.io/crates/sqlx-cli)
   and run
    ```
        sqlx database create
        sqlx migrate run
    ```
5. Install the dependencies by running the following command:

    ```
    cargo build
    ```

6. frontend build is listed in `build.rs` build file and the UI will build automatically when `cargo build` is called.Build directory is `/dist`.

7. Start the application:

    ```
    cargo run
    ```

8. Access the VaderBoard Admin Panel by visiting `http://localhost:8080` in your web browser.

9. Alternative pull the docker image from docker hub by running `docker run -p 8080:8080  v4zha/vboard-admin:latest`.
10. Alternative Build the docker container by running `build.sh` or `docker build -t vaderboard-admin . ` and run `docker run -p 8080:8080 --env-file .env vaderboard-admin`

## Features

VaderBoard Admin Panel offers the following features:

-   Live Leaderboard: Display a real-time leaderboard using Actix Actor WebSockets.
-   User Event: Manage users participating in events, including adding and deleting users, and retrieving user information.
-   Team Event: Manage teams participating in events, including creating and deleting teams, adding team members, and retrieving team information.
-   Event Management: Add, delete, and update event details.
-   Event Control: Start, stop, and update the status of events.
-   Full-text Search: Perform full-text search on events, teams, and users.
-   Vaderboard: View the top users based on scores.

    ## Admin Panel UI

    The frontend of the VaderBoard Admin Panel is built using React and TypeScript. To customize the frontend or make changes, navigate to the `/vader-admin-ui` directory and modify the source files. After making the desired changes, run `npm run build` to compile the frontend into the `/dist` directory,else autobuild is run when `cargo build` is called.

## To-dos

-   Inprove UI , Currently at a really bad state : ) .
-   Add support for knockout Events.
-   Add UI support for Team and User Avatar.
-   Add React Component for VaderBoard Client.
-   Event,User and Team Deletion
-   VaderBoard Client UI
