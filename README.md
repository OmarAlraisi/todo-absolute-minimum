# TODO: Absolute Minimum

A REST API written in Rust using only the standard library.

## Features:

- [ ] CRUD:
    - [ ] Create tasks
    - [ ] Read tasks
    - [ ] Update tasks
    - [ ] Delete tasks
- [ ] Data persistence
- [ ] Authentication:
    - [ ] Create account
    - [ ] Login
    - [ ] Logout
- [ ] Authorization:
    - [ ] Access tokens
    - [ ] Refresh tokens

## CRUD Operations:

### Create Task <u>POST</u>:

*Sample Request*:
```
POST /tasks

body {
    user_id: 1234,
    title: "Require Access Token",
    description: "Require a JWT access token for any of the crud operations to validate weather a user is allowed to perform the operation or not."
}
```

*Sample Response*:
```
HTTP/1.1 OK 200
Content-Type: application/json

{
    "id": 10,
}
```
