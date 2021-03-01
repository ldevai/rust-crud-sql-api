### Development environment setup
Create and start postgres database with docker:

    docker-compose up -f db

Connect to the database with your favorite client and create the tables in **db/initial.sql***

    Type: Postgres
    Server: localhost
    Port: 5432
    Username: demo
    Password: demo
    Database: demo

*Optional*: Start the **adminer** docker browser-based SQL client and access it at **http://localhost:8080**:

    docker-compose up -f adminer

Run the application with the command:

    cargo run


### Testing the Application ###

#### Register

    curl -H 'Content-Type: application/json' -d '{"name":"Test","email":"test@test.com","password":"abc123"}' http://localhost:8000/api/auth/register

#### Login

    curl -H 'Content-Type: application/json' -d '{"email":"test@test.com","password":"abc123"}' http://localhost:8000/api/auth/login

If everything is working, and you are using Linux/MacOS/Cygwin or have access to a bash, the one-liner below can be useful to parse the token from the response:

    TOKEN=$(curl -H 'Content-Type: application/json' -d '{"email":"test@test.com","password":"abc123"}' http://localhost:8000/api/auth/login | python -c 'import json,sys;print(json.load(sys.stdin)["access_token"])')
    echo $TOKEN
    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users
    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles

Querying the /users API without Admin role should result in an 401 Unauthorized error.

#### Change user role to admin and login again:

    UPDATE users SET role='Admin' WHERE email='test@test.com';

#### Get articles

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users

#### Create article

    curl -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"title":"Test Article","url":"test","content":"Content of full article"}' http://localhost:8000/api/articles 

#### Get first article

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles/test

#### Update Article

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles | python -c 'import json,sys;print(json.load(sys.stdin)[0]["id"])')
    echo $ID
    curl -X PUT -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"id":'\"${ID}\"',"title":"Updated Test Article","url":"test","content":"Updated content of full article","in_home":true}' http://localhost:8000/api/articles

Check article after updated:

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles/test

#### Delete article

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles | python -c 'import json,sys;print(json.load(sys.stdin)[0]["id"])')
    curl -X DELETE -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/articles/${ID}

Extra: Users API

#### Get users

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users

#### Create user

    curl -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"email":"TestUser","name":"test","password":"abc123","role":"User"}' http://localhost:8000/api/users 

#### Get new user

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users | python -c 'import json,sys;print(json.load(sys.stdin)[1]["id"])')
    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users/$ID

#### Update new user

    ID=$(curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users | python -c 'import json,sys;print(json.load(sys.stdin)[1]["id"])')
    curl -X PUT -H "Authorization: Bearer ${TOKEN}" -H 'Content-Type: application/json' -d '{"id":'\"${ID}\"',"email":"UpdatedTestUser","name":"test","role":"User"}' http://localhost:8000/api/users 

Get updated user field

    curl -H "Authorization: Bearer ${TOKEN}" http://localhost:8000/api/users | python -c 'import json,sys;print(json.load(sys.stdin)[1]["email"])'

### Building the application

#### Install sqlx-cli cargo dependency:

    cargo install sqlx-cli

#### Run cargo build

    cargo build --release

    